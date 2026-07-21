import React, { ComponentProps } from "react";

export function Label(props: ComponentProps<"label">) {
  return (
    <label
      {...props}
      className="gap-2 text-sm font-medium select-none leading-none text-foreground/80"
    >
      {props.children}
    </label>
  );
}

type Props = { label?: string } & ComponentProps<"input">;

export default function Input({ label, required, ...props }: Props) {
  return (
    <div className="flex flex-col gap-1.5">
      {label && (
        <Label htmlFor={props.id || props.name}>
          {label}
          {required && <span className="ml-1 text-destructive">*</span>}
        </Label>
      )}

      <div className="relative">
        <input
          id={props.id || props.name}
          name={props.name}
          autoComplete="off"
          required={required}
          {...props}
          className="border border-input outline-none bg-input/10 
                     focus:bg-input/30 focus:ring-2 focus:ring-primary/50
                     transition-all rounded-md h-10 w-full min-w-0 px-3 py-2 text-sm
                     disabled:opacity-50 disabled:pointer-events-none"
        />
        {props.children}
      </div>
    </div>
  );
}
