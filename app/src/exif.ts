type RawExif = Record<string, string>;

const clean = (v?: string) => v?.replace(/^"+|"+$/g, "").trim() || undefined;
const stripUnit = (v: string) =>
  v.replace(/\s*(pixels|mm|s|EV)\s*$/i, "").trim();

function parseShutterSpeed(raw?: string): string | undefined {
  const v = clean(raw);
  if (!v) return undefined;

  const frac = v.match(/1\/([\d.]+)/);
  if (frac) return `1/${Math.round(parseFloat(frac[1]))}s`;

  const num = parseFloat(v);
  return isNaN(num) ? v : `${num}s`;
}

function parseDMS(dms?: string, ref?: string): number | undefined {
  if (!dms) return undefined;
  const m = dms.match(/(\d+)\s*deg\s*(\d+)\s*min\s*([\d.]+)\s*sec/i);

  if (!m) return undefined;
  const [, deg, min, sec] = m;

  let decimal = +deg + +min / 60 + parseFloat(sec) / 3600;
  if (ref === "S" || ref === "W") decimal *= -1;

  return decimal;
}

export interface ParsedExif {
  camera?: string;
  dateTaken?: string;
  resolution?: string;
  aperture?: string;
  shutterSpeed?: string;
  iso?: string;
  focalLength?: string;
  flash?: string;
  whiteBalance?: string;
  gps?: { lat: number; lng: number; mapUrl: string; embedUrl: string };
}

export function parseExif(raw: RawExif): ParsedExif {
  const get = (key: string) => clean(raw[`x-exif-${key}`]);

  const camera = [get("make"), get("model")].filter(Boolean).join(" ");

  const w = get("pixelxdimension");
  const h = get("pixelydimension");
  const resolution = w && h ? `${stripUnit(w)} × ${stripUnit(h)}` : undefined;

  const fl = get("focallength");
  const fl35 = get("focallengthin35mmfilm");
  const focalLength = fl
    ? `${stripUnit(fl)}mm${fl35 ? ` (${stripUnit(fl35)}mm equiv)` : ""}`
    : undefined;

  const iso = get("photographicsensitivity");
  const lat = parseDMS(get("gpslatitude"), get("gpslatituderef"));
  const lng = parseDMS(get("gpslongitude"), get("gpslongituderef"));

  const gps =
    lat !== undefined && lng !== undefined
      ? {
          lat,
          lng,
          mapUrl: `https://www.google.com/maps?q=${lat},${lng}`,
          embedUrl: `https://www.google.com/maps?q=${lat},${lng}&z=15&output=embed`,
        }
      : undefined;

  return {
    camera: camera || undefined,
    dateTaken: get("datetimeoriginal"),
    resolution,
    aperture: get("fnumber"),
    shutterSpeed: parseShutterSpeed(get("exposuretime")),
    iso: iso ? `ISO ${iso}` : undefined,
    focalLength,
    flash: get("flash"),
    whiteBalance: get("whitebalance"),
    gps,
  };
}
