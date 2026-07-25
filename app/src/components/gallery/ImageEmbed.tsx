import React from "react";

type Props = { src: string; alt: string };

export default function ImageEmbed({ src, alt }: Props) {
  return (
    <div className="relative group rounded-lg overflow-hidden bg-card/20 border border-white/5">
      <img
        src={src}
        alt={alt}
        className="w-full h-auto object-cover transition-transform duration-300 group-hover:scale-105"
        loading="lazy"
      />
    </div>
  );
}
