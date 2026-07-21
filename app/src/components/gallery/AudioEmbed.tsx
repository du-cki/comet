import React, { useRef, useState } from "react";

import { Play, Pause, Disc3 } from "lucide-react";

interface AudioCardProps {
  src: string;
  name: string;
}

export default function AudioEmbed({ src, name }: AudioCardProps) {
  const audioRef = useRef<HTMLAudioElement>(null);

  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(0);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);

  const [imgFailed, setImgFailed] = useState(false);

  const thumbnailUrl = `${src}?thumbnail=true`;

  const togglePlay = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (!audioRef.current) return;

    if (isPlaying) {
      audioRef.current.pause();
    } else {
      audioRef.current.play();
    }

    setIsPlaying(!isPlaying);
  };

  const handleTimeUpdate = () => {
    if (!audioRef.current) return;
    const current = audioRef.current.currentTime;
    const total = audioRef.current.duration || 1;

    setCurrentTime(current);
    setDuration(total);
    setProgress((current / total) * 100);
  };

  const handleSeek = (e: React.MouseEvent<HTMLDivElement>) => {
    e.stopPropagation();
    if (!audioRef.current) return;

    const rect = e.currentTarget.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const percent = clickX / rect.width;

    audioRef.current.currentTime = percent * (audioRef.current.duration || 0);
  };

  const handleEnded = () => {
    setIsPlaying(false);
    setProgress(0);
    setCurrentTime(0);
  };

  const formatTime = (time: number) => {
    if (isNaN(time)) return "0:00";

    const m = Math.floor(time / 60);
    const s = Math.floor(time % 60);

    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <div className="flex flex-col bg-card border border-border rounded-xl overflow-hidden group hover:border-primary/50 transition-colors">
      <audio
        ref={audioRef}
        src={src}
        onTimeUpdate={handleTimeUpdate}
        onLoadedMetadata={handleTimeUpdate}
        onEnded={handleEnded}
        preload="none"
      />

      <div className="relative aspect-square w-full bg-muted flex items-center justify-center overflow-hidden">
        {!imgFailed ? (
          <img
            src={thumbnailUrl}
            alt={`${name} cover art`}
            onError={() => setImgFailed(true)}
            className={`w-full h-full object-cover transition-transform duration-700 ease-out ${isPlaying ? "scale-105" : "group-hover:scale-105"}`}
          />
        ) : (
          <div className="w-full h-full flex flex-col items-center justify-center bg-linear-to-br from-muted to-background">
            <Disc3
              size={48}
              className={`text-muted-foreground transition-transform duration-3000 linear ${isPlaying ? "animate-spin" : ""}`}
              strokeWidth={1}
            />
          </div>
        )}

        <div
          className={`absolute inset-0 bg-black/40 flex items-center justify-center transition-opacity duration-300 ${isPlaying ? "opacity-0 hover:opacity-100" : "opacity-0 group-hover:opacity-100"}`}
        >
          <button
            onClick={togglePlay}
            className="w-14 h-14 rounded-full bg-primary text-primary-foreground flex items-center justify-center hover:scale-110 transition-transform shadow-xl"
          >
            {isPlaying ? (
              <Pause size={24} fill="currentColor" />
            ) : (
              <Play size={24} fill="currentColor" className="ml-1" />
            )}
          </button>
        </div>
      </div>

      <div className="p-4 flex flex-col gap-3">
        <span className="font-semibold text-foreground truncate text-sm">
          {name}
        </span>

        <div className="flex flex-col gap-1.5">
          <div
            className="h-1.5 w-full bg-muted rounded-full overflow-hidden cursor-pointer group/bar relative"
            onClick={handleSeek}
          >
            <div
              className="h-full bg-primary absolute top-0 left-0 pointer-events-none"
              style={{ width: `${progress}%` }}
            />
            <div className="absolute inset-0 bg-white/10 opacity-0 group-hover/bar:opacity-100 transition-opacity" />
          </div>

          <div className="flex justify-between text-[10px] font-medium text-muted-foreground uppercase tracking-wider">
            <span>{formatTime(currentTime)}</span>
            <span>{formatTime(duration)}</span>
          </div>
        </div>
      </div>
    </div>
  );
}
