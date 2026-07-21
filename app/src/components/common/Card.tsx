import React, { ComponentProps } from "react";
import { cn } from "../../utils";

type Props = {} & ComponentProps<"div">;

export default function Card({ children, className, ...props }: Props) {
  return (
    <div
      {...props}
      className={cn(
        "bg-card/60 text-card-foreground border border-border rounded-lg",
        className,
      )}
    >
      {children}
    </div>
  );
}
