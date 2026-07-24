import React, { ComponentProps } from "react";

import { cn } from "../../utils";
import { LoaderCircle } from "lucide-react";

type Props = {
  isLoading?: boolean;
  variant?: "primary" | "secondary" | "danger" | "bordered" | "ghost" | "link";
  size?: "default" | "icon";
} & ComponentProps<"button">;

const getVariantClasses = (variant: Props["variant"]) => {
  switch (variant) {
    case "primary":
      return "bg-primary text-primary-foreground hover:bg-primary/70";
    case "secondary":
      return "bg-secondary text-secondary-foreground hover:bg-secondary/70";
    case "danger":
      return "border-destructive bg-destructive hover:bg-destructive/70 text-white";
    case "ghost":
      return "bg-transparent text-muted-foreground hover:bg-muted/50 hover:text-foreground";
    case "link":
      return "bg-transparent text-foreground underline-offset-4 hover:underline";
    case "bordered":
      return "border border-border hover:bg-muted";
    default:
      return "bg-transparent hover:bg-muted/50 hover:text-foreground";
  }
};

const getSizeClasses = (size: Props["size"]) => {
  switch (size) {
    case "icon":
      return "p-2 aspect-square";
    default:
      return "w-full px-4 py-2";
  }
};

export default function Button({
  isLoading,
  variant,
  size = "default",
  ...props
}: Props) {
  return (
    <button
      {...props}
      className={cn(
        "gap-2 font-semibold rounded-md",
        "transition-colors duration-200 cursor-pointer",
        "inline-flex items-center justify-center select-none",
        isLoading && "opacity-50 cursor-auto pointer-events-none",
        getVariantClasses(variant),
        getSizeClasses(size),
        props.className,
      )}
    >
      {props.children}

      {isLoading && (
        <LoaderCircle className="animate-spin" width={16} height={16} />
      )}
    </button>
  );
}
