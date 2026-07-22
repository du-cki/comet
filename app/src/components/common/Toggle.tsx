import React, { ComponentProps } from "react";
import { cn } from "../../utils";

type Props = {
  checked: boolean;
  onChange: (checked: boolean) => void;
} & Omit<ComponentProps<"button">, "onChange">;

export default function Toggle({
  checked,
  onChange,
  className,
  ...props
}: Props) {
  return (
    <button
      {...props}
      type="button"
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={cn(
        "inline-flex h-6 w-12 shrink-0 cursor-pointer rounded-full border-2 border-transparent",
        "transition-all duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 focus:ring-offset-background",
        checked ? "bg-primary" : "bg-muted hover:bg-muted/80",
        className,
      )}
    >
      <span
        aria-hidden="true"
        className={cn(
          "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow-md ring-0",
          "transition-transform duration-300 ease-in-out",
          checked ? "translate-x-6" : "translate-x-0",
        )}
      />
    </button>
  );
}
