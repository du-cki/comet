import React, { useState, useEffect } from "react";

import { BASE_URL, formatBytes, getAbsoluteUrl } from "../../utils";

import type { ExifHeaders, UploadsList } from "../../types";

import {
  X,
  Copy,
  Trash2,
  Calendar,
  HardDrive,
  FileType,
  Check,
} from "lucide-react";

import Button from "../common/Button";
import { api } from "../../client";

function MetaData({
  items,
}: {
  items: { icon: any; name: string; value: string }[];
}) {
  return (
    <div className="space-y-4">
      {items.map(({ icon, name, value }) => (
        <div key={name} className="flex items-center text-muted-foreground">
          {icon}

          <div className="flex flex-col">
            <span className="text-[10px] uppercase tracking-wider font-semibold text-muted-foreground">
              {name}
            </span>

            <span className="text-sm text-zinc-200">{value}</span>
          </div>
        </div>
      ))}
    </div>
  );
}

function ExtendedMetaData({
  items,
}: {
  items: { name: string; value: string }[];
}) {
  return (
    <>
      <h3 className="text-xs font-bold text-muted-foreground uppercase tracking-wider mb-4">
        Extended Metadata
      </h3>

      <div className="bg-muted/50 rounded-lg p-4 text-xs text-foreground font-mono space-y-2">
        {items.map(({ name, value }) => (
          <div key={name} className="flex justify-between">
            <span className="text-muted-foreground">{name}:</span>{" "}
            <span className="text-right">{value}</span>
          </div>
        ))}
      </div>
    </>
  );
}

type Props = {
  file: UploadsList["data"]["items"][number];
  onClose: () => void;
  onDelete: (id: string) => void;
};

export default function MediaModal({ file, onClose, onDelete }: Props) {
  const [copied, setCopied] = useState(false);
  const [exif, setExif] = useState<ExifHeaders>({});

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [onClose]);

  const url = `${BASE_URL}${file.file_url}`;

  useEffect(() => {
    api.getFileMetadata(file.file_url).then((exif) => setExif(exif));
    () => {
      setExif({});
    };
  }, []);

  const extendedItems = [
    { name: "ID", value: file.media_id },
    exif.resolution && { name: "Resolution", value: exif.resolution },
    exif.camera && { name: "Camera", value: exif.camera },
    exif.datetaken && {
      name: "Date Taken",
      value: new Date(exif.datetaken.replace(" ", "T")).toLocaleString(),
    },
    exif.aperture && { name: "Aperture", value: exif.aperture },
    exif.shutterspeed && { name: "Shutter Speed", value: exif.shutterspeed },
    exif.iso && { name: "ISO", value: exif.iso },
    exif.focallength && { name: "Focal Length", value: exif.focallength },
    exif.flash && { name: "Flash", value: exif.flash },
    exif.whitebalance && { name: "White Balance", value: exif.whitebalance },

    exif.title && { name: "Title", value: exif.title },
    exif.artist && {
      name: "Artist",
      value: exif.artist.split("; ").join(", "),
    },
    exif.album && { name: "Album", value: exif.album },
  ].filter(Boolean) as { name: string; value: string }[];

  const gps = exif.gpslatitude &&
    exif.gpslongitude && {
      mapUrl: `https://www.google.com/maps?q=${exif.gpslatitude},${exif.gpslongitude}`,
      embedUrl: `https://www.google.com/maps?q=${exif.gpslatitude},${exif.gpslongitude}&z=15&output=embed`,
    };

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(getAbsoluteUrl(url));

      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error("Failed to copy!", err);
    }
  };

  const isVideo = file.content_type.startsWith("video/");
  const isAudio = file.content_type.startsWith("audio/");

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 bg-background/80 backdrop-blur-md"
      onClick={onClose}
    >
      <div
        className="bg-card w-full max-w-6xl max-h-[90vh] h-full rounded-2xl flex flex-col md:flex-row overflow-hidden border border-border shadow-2xl"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex-1 bg-muted/30 flex items-center justify-center p-4 relative min-h-[40vh]">
          <button
            onClick={onClose}
            className="absolute top-4 left-4 p-2 rounded-full bg-background/50 text-muted-foreground hover:bg-accent hover:text-accent-foreground backdrop-blur-md transition-colors z-10"
          >
            <X size={20} />
          </button>

          {isAudio ? (
            <audio src={url} controls className="w-full max-w-md" />
          ) : isVideo ? (
            <video
              src={url}
              controls
              className="max-w-full max-h-full rounded-lg shadow-lg"
              autoPlay
            />
          ) : (
            <img
              src={url}
              alt={file.original_file_name}
              className="max-w-full max-h-full object-contain rounded-lg shadow-lg"
            />
          )}
        </div>

        <div className="w-full md:w-80 lg:w-96 bg-card flex flex-col border-t md:border-t-0 md:border-l border-border overflow-y-auto">
          <div className="p-6 flex-1">
            <h2 className="text-xl font-bold text-foreground break-all leading-tight mb-6">
              {file.original_file_name}
            </h2>

            <MetaData
              items={[
                {
                  icon: <HardDrive size={16} className="mr-3" />,
                  name: "File Size",
                  value: formatBytes(file.file_size),
                },
                {
                  icon: <FileType size={16} className="mr-3" />,
                  name: "Content Type",
                  value: file.content_type,
                },
                {
                  icon: <Calendar size={16} className="mr-3" />,
                  name: "Uploaded",
                  value: new Date(file.uploaded_at * 1000).toLocaleString(),
                },
              ]}
            />

            <div className="mt-8 pt-6 border-t border-border space-y-5">
              {gps && (
                <div>
                  <h3 className="text-xs font-bold text-muted-foreground uppercase tracking-wider mb-3">
                    Location
                  </h3>

                  <a
                    href={gps.mapUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="block rounded-lg overflow-hidden border border-border hover:opacity-90 transition-opacity"
                  >
                    <iframe
                      src={gps.embedUrl}
                      className="w-full h-40 pointer-events-none"
                      loading="lazy"
                      title="Photo location"
                    />
                  </a>

                  <p className="text-xs text-muted-foreground mt-2">
                    {exif.gpslatitude!}, {exif.gpslongitude}
                  </p>
                </div>
              )}

              <ExtendedMetaData items={extendedItems} />
            </div>
          </div>

          <div className="p-6 border-t border-border bg-card/80 backdrop-blur-xl flex gap-3">
            <Button
              variant="secondary"
              onClick={handleCopy}
              className="flex-1 flex items-center justify-center gap-2"
            >
              {copied ? (
                <Check size={16} className="text-emerald-400" />
              ) : (
                <Copy size={16} />
              )}
              {copied ? "Copied!" : "Copy Link"}
            </Button>

            <Button
              variant="danger"
              onClick={() => onDelete(file.media_id)}
              className="flex-1 flex items-center justify-center gap-2"
            >
              <Trash2 size={16} />
              Delete
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
