import React, { useState, useRef, useEffect } from "react";

import { BASE_URL, cn, TOKEN_NAME } from "../../utils";

import {
  ImageUp,
  File as FileIcon,
  X,
  CheckCircle2,
  Loader2,
  Trash,
  AlertCircle,
  Clock,
} from "lucide-react";
import Button from "../common/Button";

type UploadTask = {
  id: string;
  file: File;
  progress: number;
  status: "pending" | "uploading" | "completed" | "error";
};

export default function UploadDropdown() {
  const [isOpen, setIsOpen] = useState(false);
  const [uploads, setUploads] = useState<UploadTask[]>([]);

  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const dragCounter = useRef(0);

  const fileInputRef = useRef<HTMLInputElement>(null);

  const addFiles = (files: File[]) => {
    if (files.length === 0) return;

    const newTasks: UploadTask[] = files.map((file) => ({
      id: crypto.randomUUID(),
      file,
      progress: 0,
      status: "pending",
    }));

    setUploads((prev) => [...prev, ...newTasks]);
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(e.target.files || []);
    addFiles(files);

    if (fileInputRef.current) fileInputRef.current.value = "";
  };

  const handleDragEnter = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();

    dragCounter.current += 1;
    if (e.dataTransfer.types.includes("Files")) {
      setIsDraggingOver(true);
    }
  };

  const handleDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();

    if (e.dataTransfer.types.includes("Files")) {
      e.dataTransfer.dropEffect = "copy";
    }
  };

  const handleDragLeave = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();

    dragCounter.current -= 1;
    if (dragCounter.current <= 0) {
      dragCounter.current = 0;
      setIsDraggingOver(false);
    }
  };

  const handleDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();

    dragCounter.current = 0;
    setIsDraggingOver(false);

    const files = Array.from(e.dataTransfer.files || []);
    addFiles(files);
  };

  useEffect(() => {
    const isUploading = uploads.some((u) => u.status === "uploading");
    if (isUploading) return;

    const nextTask = uploads.find((u) => u.status === "pending");
    if (nextTask) {
      performUpload(nextTask);
    }
  }, [uploads]);

  useEffect(() => {
    if (!isOpen) return;

    const handlePaste = (e: ClipboardEvent) => {
      const items = e.clipboardData?.items;
      if (!items) return;

      const files: File[] = [];
      for (const item of Array.from(items)) {
        if (item.kind === "file") {
          const file = item.getAsFile();
          if (file) files.push(file);
        }
      }

      if (files.length === 0) return;

      e.preventDefault();
      addFiles(files);
    };

    window.addEventListener("paste", handlePaste);
    return () => window.removeEventListener("paste", handlePaste);
  }, [isOpen]);

  const performUpload = (task: UploadTask) => {
    updateTask(task.id, { status: "uploading" });

    const xhr = new XMLHttpRequest();
    const formData = new FormData();
    formData.append("file", task.file);

    const token = localStorage.getItem(TOKEN_NAME);

    xhr.open("POST", `${BASE_URL}/upload`, true);
    xhr.setRequestHeader("Authorization", `Bearer ${token}`);

    xhr.upload.onprogress = (event) => {
      if (event.lengthComputable) {
        const percent = Math.round((event.loaded / event.total) * 100);
        updateTask(task.id, { progress: percent });
      }
    };

    xhr.onload = () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        try {
          const response = JSON.parse(xhr.responseText);
          const result = response[0];

          if (result && result.error) {
            updateTask(task.id, { status: "error" });

            console.error(
              `upload failed for ${result.original_file_name}:`,
              result.error,
            );

            return;
          }

          updateTask(task.id, { status: "completed", progress: 100 });
        } catch (e) {
          updateTask(task.id, { status: "error" });
          console.error("failed to parse JSON response:", e);
        }
      } else {
        updateTask(task.id, { status: "error" });
        console.error("upload failed:", xhr.responseText);
      }
    };

    xhr.onerror = () => {
      updateTask(task.id, { status: "error" });
    };

    xhr.send(formData);
  };

  const updateTask = (id: string, updates: Partial<UploadTask>) => {
    setUploads((prev) =>
      prev.map((task) => (task.id === id ? { ...task, ...updates } : task)),
    );
  };

  const clearCompleted = () => {
    setUploads((prev) =>
      prev.filter((u) => u.status === "pending" || u.status === "uploading"),
    );
  };

  const hasUploads = uploads.length > 0;

  return (
    <div className="relative">
      <Button
        variant="primary"
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2"
      >
        <ImageUp size={18} />
        Upload
      </Button>

      {isOpen && (
        <div className="absolute right-0 top-full mt-3 w-80 md:w-96 bg-background border border-border rounded-2xl shadow-2xl z-40 overflow-hidden flex flex-col max-h-[80vh]">
          <div className="px-4 py-3 border-b border-border flex justify-end items-center bg-muted/30">
            <div className="flex items-center gap-2">
              {hasUploads && (
                <button
                  onClick={clearCompleted}
                  className="text-zinc-500 hover:text-zinc-300 transition-colors mr-2"
                >
                  <Trash size={16} />
                </button>
              )}
              <button
                onClick={() => setIsOpen(false)}
                className="text-zinc-500 hover:text-white transition-colors"
              >
                <X size={16} />
              </button>
            </div>
          </div>

          <div className="p-4 flex flex-col gap-4 overflow-y-auto">
            <div className="flex flex-col gap-3 shrink-0">
              <div
                onClick={() => fileInputRef.current?.click()}
                onDragEnter={handleDragEnter}
                onDragOver={handleDragOver}
                onDragLeave={handleDragLeave}
                onDrop={handleDrop}
                className={cn(
                  "border-2 border-dashed border-border hover:border-primary/50 hover:bg-primary/5 rounded-xl p-6 flex flex-col items-center justify-center gap-2 cursor-pointer transition-colors group",
                  isDraggingOver
                    ? "border-primary bg-primary/10"
                    : "border-border hover:border-primary/50 hover:bg-primary/5",
                )}
              >
                <div
                  className={cn(
                    "w-10 h-10 rounded-full bg-muted group-hover:bg-primary/20 flex items-center justify-center text-muted-foreground group-hover:text-primary transition-colors",
                    isDraggingOver
                      ? "bg-primary/20 text-primary"
                      : "bg-muted group-hover:bg-primary/20 text-muted-foreground group-hover:text-primary",
                  )}
                >
                  <ImageUp size={20} />
                </div>

                <span className="text-sm font-medium text-foreground mt-2">
                  {isDraggingOver ? "Drop files to upload" : "Click to browse"}
                </span>

                <span className="text-xs text-foreground">
                  Supports JPG, PNG, MP4, FLAC
                </span>
              </div>

              <input
                type="file"
                multiple
                className="hidden"
                ref={fileInputRef}
                onChange={handleFileChange}
              />
            </div>

            {hasUploads && (
              <div className="flex flex-col gap-2 mt-2 border-t border-border pt-4">
                {uploads.map((task) => (
                  <div
                    key={task.id}
                    className="bg-card border border-border rounded-lg p-3 flex flex-col gap-2"
                  >
                    <div className="flex items-start gap-3">
                      <div className="w-8 h-8 rounded bg-muted flex items-center justify-center text-muted-foreground shrink-0">
                        <FileIcon size={16} />
                      </div>

                      <div className="flex flex-col flex-1 min-w-0">
                        <span className="text-sm font-medium text-foreground truncate w-full">
                          {task.file.name}
                        </span>

                        <div className="flex items-center gap-2 text-xs font-semibold mt-0.5">
                          {task.status === "pending" && (
                            <span className="text-muted-foreground flex items-center gap-1">
                              <Clock size={12} /> Waiting...
                            </span>
                          )}

                          {task.status === "uploading" && (
                            <span className="text-primary">
                              Uploading {task.progress}%
                            </span>
                          )}

                          {task.status === "completed" && (
                            <span className="text-emerald-500">Complete</span>
                          )}

                          {task.status === "error" && (
                            <span className="text-destructive">Failed</span>
                          )}
                        </div>
                      </div>

                      <div className="shrink-0 mt-1">
                        {task.status === "uploading" && (
                          <Loader2
                            size={16}
                            className="text-primary animate-spin"
                          />
                        )}
                        {task.status === "completed" && (
                          <CheckCircle2
                            size={16}
                            className="text-emerald-500"
                          />
                        )}
                        {task.status === "error" && (
                          <AlertCircle size={16} className="text-destructive" />
                        )}
                      </div>
                    </div>

                    {task.status !== "pending" && (
                      <div className="w-full h-1 bg-muted rounded-full overflow-hidden">
                        <div
                          className={`h-full transition-all duration-300 ease-out 
                            ${task.status === "completed" ? "bg-emerald-500" : ""}
                            ${task.status === "error" ? "bg-destructive" : ""}
                            ${task.status === "uploading" ? "bg-primary" : ""}
                          `}
                          style={{
                            width: `${task.status === "error" ? 100 : task.progress}%`,
                          }}
                        />
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
