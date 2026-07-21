import React, { ComponentProps } from "react";

import { cn } from "../../utils";
import { LoaderCircle } from "lucide-react";

type Props = {
  isLoading?: boolean;
  variant?: "primary" | "secondary" | "danger" | "ghost" | "link";
} & ComponentProps<"button">;

const getVariantClasses = (variant: Props["variant"]) => {
  switch (variant) {
    case "primary":
      return "bg-primary text-primary-foreground hover:bg-primary/70";
    case "secondary":
      return "bg-secondary text-secondary-foreground hover:bg-secondary/70";
    case "danger":
      return "border-destructive bg-destructive hover:bg-destructive/70 text-white";
    default:
      return "hover:bg-muted/50 hover:text-foreground";
  }
};

export default function Button({ isLoading, variant, ...props }: Props) {
  return (
    <button
      {...props}
      className={cn(
        "w-full px-4 py-2 gap-2 font-semibold rounded-lg",
        "transition-colors duration-200 cursor-pointer",
        "inline-flex items-center justify-center select-none",
        isLoading && "opacity-50 cursor-auto pointer-events-none",
        getVariantClasses(variant),
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
