import React, { useCallback, useEffect, useRef, useState } from "react";

import { useWebSocket } from "../providers/WebSocketProvider";

import { BASE_URL } from "../utils";

import { api } from "../client";
import type { File, FileDelete, FilesList, FileUpload } from "../types";

import { LoaderCircle } from "lucide-react";

import VideoEmbed from "../components/gallery/VideoEmbed";
import AudioEmbed from "../components/gallery/AudioEmbed";
import MediaModal from "../components/gallery/MediaModal";
import UploadDropdown from "../components/gallery/UploadDropdown";
import GenericEmbed from "../components/gallery/GenericEmbed";
import ImageEmbed from "../components/gallery/ImageEmbed";

export default function Gallery() {
  const [files, setFiles] = useState<File[]>([]);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);

  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [hasMore, setHasMore] = useState(true);

  const { ws, status } = useWebSocket();

  const cursorRef = useRef<string | null>(null);
  const isFetchingRef = useRef(false);
  const isFirstPageRef = useRef(true);

  const sentinelRef = useRef<HTMLDivElement | null>(null);

  const fetchPage = useCallback(
    (cursor: string | null) => {
      if (!ws || status !== "connected") return;
      if (isFetchingRef.current) return;
      if (!cursor && !isFirstPageRef.current) {
        return;
      }

      isFetchingRef.current = true;
      isFirstPageRef.current = cursor === null;
      if (cursor !== null) setIsLoadingMore(true);

      api
        .getFiles({ cursor, limit: 20 })
        .then((files) =>
          setFiles((old) =>
            isFirstPageRef.current ? files.items : [...old, ...files.items],
          ),
        );
    },
    [ws, status],
  );

  useEffect(() => {
    if (!ws || status !== "connected") return;

    cursorRef.current = null;
    setFiles([]);
    setHasMore(true);
    fetchPage(null);

    const handleMessage = (ev: MessageEvent) => {
      const e = JSON.parse(ev.data);

      if (e.type === "FileUpload") {
        const { data } = e as FileUpload;

        setFiles((oldFiles) => [data, ...oldFiles]);
      } else if (e.type === "FileDelete") {
        const { data } = e as FileDelete;

        setFiles((oldFiles) =>
          oldFiles.filter((file) => file.media_id !== data),
        );
      }
    };

    ws.addEventListener("message", handleMessage);
    return () => {
      ws.removeEventListener("message", handleMessage);
    };
  }, [ws, status, fetchPage]);

  useEffect(() => {
    const sentinel = sentinelRef.current;
    if (!sentinel) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasMore && !isFetchingRef.current) {
          fetchPage(cursorRef.current);
        }
      },
      { rootMargin: "400px" },
    );

    observer.observe(sentinel);
    return () => observer.disconnect();
  }, [hasMore, fetchPage]);

  const handleDelete = async (id: string) => {
    setSelectedFile(null);

    const req = await api.deleteFile(id);

    if (req.status === 401) {
      window.location.reload();
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-4 md:px-6 border-b border-white/5 mb-4">
        <div>
          <h1 className="text-2xl font-bold text-white tracking-tight">
            Gallery
          </h1>

          <p className="text-sm text-zinc-500">
            Manage and view your uploaded files
          </p>
        </div>

        <UploadDropdown />
      </div>

      <div className="columns-2 md:columns-3 lg:columns-4 space-y-4 p-4 select-none">
        {files.map((file) => {
          const isVideo = file.content_type.startsWith("video/");
          const isAudio = file.content_type.startsWith("audio/");
          const isImage = file.content_type.startsWith("image/");

          const url = `${BASE_URL}${file.file_url}`;

          return (
            <div
              key={file.media_id}
              className="break-inside-avoid"
              onClick={() => setSelectedFile(file)}
            >
              {isImage ? (
                <ImageEmbed
                  src={url.replace("view", "thumb")}
                  alt={file.original_file_name}
                />
              ) : isVideo ? (
                <VideoEmbed src={url} />
              ) : isAudio ? (
                <AudioEmbed
                  name={file.original_file_name}
                  src={url}
                  thumbnail={url.replace("view", "thumb")}
                />
              ) : (
                <GenericEmbed
                  name={file.original_file_name}
                  content_type={file.content_type}
                />
              )}
            </div>
          );
        })}
      </div>

      {hasMore && (
        <div ref={sentinelRef} className="flex justify-center py-6">
          {isLoadingMore && (
            <LoaderCircle
              className="animate-spin text-muted-foreground"
              size={24}
            />
          )}
        </div>
      )}

      {selectedFile && (
        <MediaModal
          file={selectedFile}
          onClose={() => setSelectedFile(null)}
          onDelete={handleDelete}
        />
      )}
    </div>
  );
}
