import React, { ComponentProps } from "react";
import { cn } from "../../utils";

type Props = { label?: string } & ComponentProps<"div">;

export default function Card({ label, children, className, ...props }: Props) {
  return (
    <div
      {...props}
      className={cn(
        "bg-card/60 text-card-foreground border border-border rounded-lg p-6",
        className,
      )}
    >
      {label && (
        <h3 className="select-none text-xs font-bold text-muted-foreground uppercase tracking-widest mb-6">
          {label}
        </h3>
      )}

      {children}
    </div>
  );
}
