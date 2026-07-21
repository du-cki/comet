import React, { ComponentProps } from "react";

import { MoveDown, MoveUp } from "lucide-react";
import { cn } from "../utils";

interface StatProps {
  name: string;
  value: string;
  trend?: {
    value: string;
    type: "up" | "down" | "neutral";
  };
}

export function Stat({ name, value, trend }: StatProps) {
  return (
    <div className="flex-1 flex flex-col justify-center p-4 sm:p-5">
      <div className="flex items-baseline gap-2">
        <span className="text-xl sm:text-2xl font-bold text-white">
          {value}
        </span>

        {trend && (
          <span
            className={`flex items-center text-xs font-medium ${
              trend.type === "up"
                ? "text-emerald-500"
                : trend.type === "down"
                  ? "text-rose-500"
                  : "text-gray-400"
            }`}
          >
            {trend.type === "up" && <MoveUp className="w-3 h-3 mr-0.5" />}
            {trend.type === "down" && <MoveDown className="w-3 h-3 mr-0.5" />}

            {trend.value}
          </span>
        )}
      </div>

      <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider mt-1">
        {name}
      </span>
    </div>
  );
}

type ContainerProps = {} & ComponentProps<"div">;

export function StatContainer({ children, ...props }: ContainerProps) {
  return (
    <div
      {...props}
      className={cn(
        `flex flex-col md:flex-row 
        bg-card/60 rounded-lg overflow-hidden
        divide-y-2 md:divide-y-0 md:divide-x-2 divide-white/5 
        border-2 border-white/5`,
        props.className,
      )}
    >
      {children}
    </div>
  );
}
