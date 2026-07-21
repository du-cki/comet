import React, { useState, useEffect } from "react";

import { BASE_URL, formatBytes } from "../../utils";

import type { UploadsList } from "../../types";

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

interface MediaModalProps {
  file: UploadsList["data"]["items"][number];
  onClose: () => void;
  onDelete: (id: string) => void;
}

export function MediaModal({ file, onClose, onDelete }: MediaModalProps) {
  const [copied, setCopied] = useState(false);
  const [exif, setExif] = useState<Record<string, string>>({});

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [onClose]);

  const url = `${BASE_URL}${file.file_url}`;

  useEffect(() => {
    fetch(url, { method: "HEAD" }).then((r) => {
      const { headers } = r;
      const exif: Record<string, string> = {};

      headers.forEach((value, key) => {
        if (key.startsWith("x-exif") || key.startsWith("x-audio")) {
          exif[key] = value;
        }
      });

      setExif(exif);
    });
  }, []);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(url);
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
      className="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 bg-black/80 backdrop-blur-md"
      onClick={onClose}
    >
      <div
        className="bg-[#09090b] w-full max-w-6xl max-h-[90vh] h-full rounded-2xl flex flex-col md:flex-row overflow-hidden border border-white/10 shadow-2xl"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex-1 bg-black/40 flex items-center justify-center p-4 relative min-h-[40vh]">
          <button
            onClick={onClose}
            className="absolute top-4 left-4 p-2 rounded-full bg-black/50 text-white hover:bg-white/20 backdrop-blur-md transition-colors z-10"
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

        <div className="w-full md:w-80 lg:w-96 bg-[#09090b] flex flex-col border-t md:border-t-0 md:border-l border-white/10 overflow-y-auto">
          <div className="p-6 flex-1">
            <h2 className="text-xl font-bold text-white break-all leading-tight mb-6">
              {file.original_file_name}
            </h2>

            <div className="space-y-4">
              <div className="flex items-center text-zinc-400">
                <HardDrive size={16} className="mr-3" />

                <div className="flex flex-col">
                  <span className="text-[10px] uppercase tracking-wider font-semibold text-zinc-500">
                    File Size
                  </span>

                  <span className="text-sm text-zinc-200">
                    {formatBytes(file.file_size)}
                  </span>
                </div>
              </div>

              <div className="flex items-center text-zinc-400">
                <FileType size={16} className="mr-3" />

                <div className="flex flex-col">
                  <span className="text-[10px] uppercase tracking-wider font-semibold text-zinc-500">
                    Content Type
                  </span>

                  <span className="text-sm text-zinc-200">
                    {file.content_type}
                  </span>
                </div>
              </div>

              <div className="flex items-center text-zinc-400">
                <Calendar size={16} className="mr-3" />

                <div className="flex flex-col">
                  <span className="text-[10px] uppercase tracking-wider font-semibold text-zinc-500">
                    Uploaded
                  </span>

                  <span className="text-sm text-zinc-200">
                    {new Date(file.uploaded_at * 1000).toLocaleString()}
                  </span>
                </div>
              </div>
            </div>

            <div className="mt-8 pt-6 border-t border-white/5">
              <h3 className="text-xs font-bold text-zinc-500 uppercase tracking-wider mb-4">
                Extended Metadata
              </h3>

              <div className="bg-white/5 rounded-lg p-4 text-xs text-zinc-400 font-mono space-y-2">
                <div className="flex justify-between">
                  <span className="text-zinc-500">ID:</span>{" "}
                  <span>{file.media_id}</span>
                </div>

                <div className="flex justify-between">
                  <span className="text-zinc-500">Resolution:</span>{" "}
                  <span>--</span>
                </div>

                <div className="flex justify-between">
                  <span className="text-zinc-500">Camera:</span> <span>--</span>
                </div>
              </div>
            </div>
          </div>

          <div className="p-6 border-t border-white/10 bg-[#09090b]/80 backdrop-blur-xl flex gap-3">
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
