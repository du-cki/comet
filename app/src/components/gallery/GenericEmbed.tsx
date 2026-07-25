import { File } from "lucide-react";
import React from "react";

type Props = {
  name: string;
  content_type: string;
};

export default function GenericEmbed({ name, content_type }: Props) {
  return (
    <div className="flex flex-col items-center justify-center bg-card border border-border rounded-xl p-8 group hover:border-primary/50 transition-colors min-h-48">
      <File
        size={32}
        className="text-muted-foreground mb-3"
        strokeWidth={1.5}
      />

      <span className="text-sm font-medium text-foreground text-center line-clamp-2 break-all px-2">
        {name}
      </span>

      <span className="text-[10px] text-muted-foreground/70 uppercase tracking-wider mt-1.5">
        {content_type.split("/")[1] || "FILE"}
      </span>
    </div>
  );
}
