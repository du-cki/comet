import React, { ComponentProps } from "react";

import { MoveDown, MoveUp } from "lucide-react";
import { cn } from "../utils";
import Card from "./common/Card";

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
        <span className="text-xl sm:text-2xl font-bold text-card-foreground">
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

      <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mt-1">
        {name}
      </span>
    </div>
  );
}

type ContainerProps = {} & ComponentProps<"div">;

export function StatContainer({ children, ...props }: ContainerProps) {
  return (
    <Card
      {...props}
      className={cn(
        "flex flex-col md:flex-row overflow-hidden",
        "p-0 divide-x-0 divide-y md:divide-x md:divide-y-0 divide-border",
        props.className,
      )}
    >
      {children}
    </Card>
  );
}
