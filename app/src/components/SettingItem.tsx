import React from "react";
import Toggle from "./common/Toggle";

interface SettingItemProps {
  label: string;
  description: string;

  checked?: boolean;
  onToggle?: (checked: boolean) => void;

  value?: number;
  onValueChange?: (value: number) => void;
  unit?: string;
}

export function SettingItem({
  label,
  description,
  checked,
  onToggle,
  value,
  onValueChange,
  unit,
}: SettingItemProps) {
  const hasToggle = onToggle !== undefined;
  const hasInput = onValueChange !== undefined;

  const isInputBelow = hasToggle && hasInput;
  const isInputEnabled = hasToggle ? checked : true;

  const inputElement = hasInput && (
    <div
      className={`flex items-center gap-2 ${
        isInputBelow ? "transition-opacity duration-200" : ""
      } ${!isInputEnabled ? "opacity-40 pointer-events-none" : "opacity-100"}`}
    >
      <input
        type="number"
        min={1}
        value={value}
        onChange={(e) => onValueChange(Number(e.target.value))}
        disabled={!isInputEnabled}
        className="bg-muted text-foreground border border-border rounded-md px-3 py-1.5 text-sm w-24 focus:outline-none focus:ring-2 focus:ring-accent transition-colors"
      />
      {unit && (
        <span className="text-sm font-medium text-muted-foreground">
          {unit}
        </span>
      )}
    </div>
  );

  return (
    <div
      className={`flex justify-between ${isInputBelow ? "items-start" : "items-center"}`}
    >
      <div className="flex flex-col">
        <span className="text-sm font-semibold tracking-wide text-foreground">
          {label}
        </span>
        <span
          className={`text-xs text-muted-foreground mt-1 ${isInputBelow ? "mb-3" : ""}`}
        >
          {description}
        </span>

        {isInputBelow && inputElement}
      </div>

      <div className="flex items-center gap-4">
        {!isInputBelow && hasInput && inputElement}
        {hasToggle && <Toggle checked={checked || false} onChange={onToggle} />}
      </div>
    </div>
  );
}
