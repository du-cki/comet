use std::{collections::HashMap, io::Seek, sync::OnceLock};

use regex::Regex;
use serde::{Deserialize, Serialize};

use lofty::{file::TaggedFileExt, read_from, tag::Accessor};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GpsData {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ParsedMetadata {
    pub camera: Option<String>,
    pub date_taken: Option<String>,
    pub resolution: Option<String>,
    pub aperture: Option<String>,
    pub shutter_speed: Option<String>,
    pub iso: Option<String>,
    pub focal_length: Option<String>,
    pub flash: Option<String>,
    pub white_balance: Option<String>,

    #[serde(skip_serializing)]
    pub gps: Option<GpsData>,

    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

fn clean(v: Option<&String>) -> Option<String> {
    v.map(|s| s.trim_matches('"').trim().to_string())
        .filter(|s| !s.is_empty())
}

fn strip_unit(v: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"(?i)\s*(pixels|mm|s|EV)\s*$").unwrap());
    re.replace(v, "").trim().to_string()
}

fn parse_shutter_speed(raw: Option<&String>) -> Option<String> {
    let v = clean(raw)?;

    static RE_FRAC: OnceLock<Regex> = OnceLock::new();
    let re_frac = RE_FRAC.get_or_init(|| Regex::new(r"1/([\d.]+)").unwrap());

    if let Some(caps) = re_frac.captures(&v) {
        if let Ok(num) = caps[1].parse::<f64>() {
            return Some(format!("1/{}s", num.round()));
        }
    }

    if let Ok(num) = v.parse::<f64>() {
        return Some(format!("{}s", num));
    }

    Some(v)
}

fn parse_dms(dms: Option<&String>, ref_val: Option<&String>) -> Option<f64> {
    let dms_str = dms?;

    static RE_DMS: OnceLock<Regex> = OnceLock::new();
    let re = RE_DMS
        .get_or_init(|| Regex::new(r"(?i)(\d+)\s*deg\s*(\d+)\s*min\s*([\d.]+)\s*sec").unwrap());

    let caps = re.captures(dms_str)?;

    let deg: f64 = caps[1].parse().ok()?;
    let min: f64 = caps[2].parse().ok()?;
    let sec: f64 = caps[3].parse().ok()?;

    let mut decimal = deg + (min / 60.0) + (sec / 3600.0);

    if let Some(r) = ref_val {
        let r_upper = r.to_uppercase();
        if r_upper.contains('S') || r_upper.contains('W') {
            decimal *= -1.0;
        }
    }

    Some(decimal)
}

pub async fn extract_raw_metadata(file_path: String) -> HashMap<String, String> {
    tracing::info!("extracting metadata for: {}", file_path);

    tokio::task::spawn_blocking(move || {
        let mut map = HashMap::new();

        let mut file = match std::fs::File::open(&file_path) {
            Ok(f) => f,
            Err(_) => return map,
        };

        if let Ok(tagged_file) = read_from(&mut file) {
            if let Some(tag) = tagged_file.primary_tag() {
                let mut add_audio_header = |key: &str, value: Option<&str>| {
                    if let Some(val) = value {
                        let header_key = key.to_string();
                        let safe_val = val.replace('\n', " ").replace('\r', "");

                        map.insert(header_key, safe_val);
                    }
                };

                add_audio_header("artist", tag.artist().as_deref());
                add_audio_header("title", tag.title().as_deref());
                add_audio_header("album", tag.album().as_deref());
                add_audio_header("genre", tag.genre().as_deref());

                tracing::info!("added music metadata");
            }
        }

        if let Err(_) = file.rewind() {
            tracing::info!("failed to rewind file, early return");
            return map;
        };

        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();

        if let Ok(exif_data) = exifreader.read_from_container(&mut bufreader) {
            for field in exif_data.fields() {
                let header_key = field.tag.to_string().to_lowercase();
                let val_str = field.display_value().with_unit(&exif_data).to_string();
                let safe_val = val_str.replace('\n', " ").replace('\r', "");

                map.insert(header_key, safe_val);
            }

            tracing::info!("added exif metadata");
        }

        map
    })
    .await
    .unwrap_or_default()
}

pub fn parse_metadata(raw: &HashMap<String, String>) -> ParsedMetadata {
    let make = clean(raw.get("make"));
    let model = clean(raw.get("model"));
    let camera = match (make, model) {
        (Some(m1), Some(m2)) => Some(format!("{} {}", m1, m2)),
        (Some(m), None) | (None, Some(m)) => Some(m),
        _ => None,
    };

    let w = clean(raw.get("pixelxdimension"));
    let h = clean(raw.get("pixelydimension"));
    let resolution = if let (Some(w_val), Some(h_val)) = (w, h) {
        Some(format!("{} x {}", strip_unit(&w_val), strip_unit(&h_val)))
    } else {
        None
    };

    let fl = clean(raw.get("focallength"));
    let fl35 = clean(raw.get("focallengthin35mmfilm"));
    let focal_length = if let Some(fl_val) = fl {
        if let Some(fl35_val) = fl35 {
            Some(format!(
                "{}mm ({}mm equiv)",
                strip_unit(&fl_val),
                strip_unit(&fl35_val)
            ))
        } else {
            Some(format!("{}mm", strip_unit(&fl_val)))
        }
    } else {
        None
    };

    let iso = clean(raw.get("photographicsensitivity")).map(|v| format!("ISO {}", v));

    let lat = parse_dms(raw.get("gpslatitude"), raw.get("gpslatituderef"));
    let lng = parse_dms(raw.get("gpslongitude"), raw.get("gpslongituderef"));

    let gps = if let (Some(lat_val), Some(lng_val)) = (lat, lng) {
        Some(GpsData {
            lat: lat_val,
            lng: lng_val,
        })
    } else {
        None
    };

    ParsedMetadata {
        camera,
        date_taken: clean(raw.get("datetimeoriginal")),
        resolution,
        aperture: clean(raw.get("fnumber")),
        shutter_speed: parse_shutter_speed(raw.get("exposuretime")),
        iso,
        focal_length,
        flash: clean(raw.get("flash")),
        white_balance: clean(raw.get("whitebalance")),
        gps,

        title: clean(raw.get("title")),
        artist: clean(raw.get("artist")),
        album: clean(raw.get("album")),
    }
}
