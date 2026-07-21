import { useRef, useState } from "react";
import { Play, Volume2, VolumeX } from "lucide-react";

interface VideoEmbedProps {
  src: string;
  poster?: string;
  className?: string;
}

export default function VideoEmbed({
  src,
  poster,
  className = "",
}: VideoEmbedProps) {
  const videoRef = useRef<HTMLVideoElement>(null);

  const [isHovered, setIsHovered] = useState(false);
  const [isMuted, setIsMuted] = useState(true);

  const handleMouseEnter = async () => {
    setIsHovered(true);
    if (videoRef.current) {
      try {
        await videoRef.current.play();
      } catch (error) {
        console.log("playback interrupted or blocked:", error);
      }
    }
  };

  const handleMouseLeave = () => {
    setIsHovered(false);

    if (videoRef.current) {
      videoRef.current.pause();
      videoRef.current.currentTime = 0;
    }
  };

  return (
    <div
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      className={`relative rounded-lg overflow-hidden cursor-pointer group bg-card/20 ${className}`}
    >
      <video
        ref={videoRef}
        src={src}
        poster={poster}
        muted={isMuted}
        playsInline
        loop
        className="w-full h-auto object-cover transition-transform duration-500 group-hover:scale-102"
      />

      {!isHovered && (
        <div className="absolute top-3 right-3 p-1.5 rounded-full bg-black/60 text-white backdrop-blur-sm pointer-events-none opacity-80 group-hover:opacity-0 transition-opacity">
          <Play size={14} fill="currentColor" />
        </div>
      )}

      {isHovered && (
        <div className="absolute bottom-3 right-3 z-10">
          <button
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();

              setIsMuted(!isMuted);
            }}
            className="p-2 rounded-full bg-black/60 text-white hover:bg-black/80 backdrop-blur-sm transition-colors"
          >
            {isMuted ? <VolumeX size={16} /> : <Volume2 size={16} />}
          </button>
        </div>
      )}
    </div>
  );
}
