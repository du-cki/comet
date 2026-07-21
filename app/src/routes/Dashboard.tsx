import React, { useEffect, useState } from "react";

import { useWebSocket } from "../providers/WebSocketProvider";

import { formatBytes } from "../utils";
import type { DashboardStats } from "../types";

import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip } from "recharts";

import Card from "../components/common/Card";
import { Stat, StatContainer } from "../components/Stats";

const matchFileTypeColour = (type: string) => {
  switch (type) {
    case "Images":
      return "#10b981";
    case "Videos":
      return "#3b82f6";
    case "Audio":
      return "#8b5cf6";
    case "Documents":
      return "#3f3f46";
    default:
      return "#3f3f46";
  }
};

function StorageBreakdown({
  total_files,
  file_types,
}: {
  total_files: number;
  file_types: DashboardStats["data"]["file_types"];
}) {
  const perc = (count: number) =>
    total_files > 0 ? (count / total_files) * 100 : 0;

  const stats = file_types
    .map((stat) => ({
      name: stat.name,
      value: perc(stat.amount),
      amount: stat.amount,
      color: matchFileTypeColour(stat.name),
    }))
    .filter(({ value }) => value > 0);

  return (
    <Card className="p-6">
      <h2 className="text-sm font-medium text-muted-foreground uppercase tracking-wide mb-4">
        Storage Breakdown
      </h2>

      <div className="h-50">
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              data={stats}
              cx="50%"
              cy="50%"
              innerRadius="65%"
              outerRadius="90%"
              paddingAngle={3}
              dataKey="value"
              stroke="none"
            >
              {stats.map((entry) => (
                <Cell key={entry.name} fill={entry.color} />
              ))}
            </Pie>

            <Tooltip formatter={(item) => `${Number(item).toFixed(0)}%`} />
          </PieChart>
        </ResponsiveContainer>
      </div>

      <div className="space-y-1.5">
        {stats.map((item) => (
          <div
            key={item.name}
            className="flex items-center justify-between text-sm"
          >
            <div className="flex items-center gap-3">
              <div
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: item.color }}
              />

              <span className="text-muted-foreground">{item.name}</span>
            </div>

            <span className="font-medium text-foreground">{item.amount}</span>
          </div>
        ))}
      </div>
    </Card>
  );
}

export default function Dashboard() {
  const [stats, setStats] = useState<DashboardStats["data"]>({
    total_files: 0,
    total_files_trend: 0,
    storage_used_bytes: 0,
    storage_used_bytes_trend: 0,
    views: 0,
    average_file_size_bytes: 0,
    file_types: [],
  });

  const { ws, status } = useWebSocket();

  useEffect(() => {
    if (!ws || status !== "connected") return;

    ws.send(JSON.stringify({ action: "GetStats" }));

    const handleMessage = (ev: MessageEvent) => {
      const e = JSON.parse(ev.data);
      if (e.type === "DashboardStats") {
        setStats((e as DashboardStats).data);
      }
    };

    ws.addEventListener("message", handleMessage);
    return () => {
      ws.removeEventListener("message", handleMessage);
    };
  }, [ws, status]);

  return (
    <div className="space-y-6">
      <StatContainer className="w-full">
        <Stat
          name="Files"
          value={stats.total_files.toString()}
          trend={{
            value: stats.total_files_trend.toString(),
            type: "up",
          }}
        />

        <Stat
          name="Storage Used"
          value={formatBytes(Number(stats.storage_used_bytes))}
          trend={{
            value: formatBytes(Number(stats.storage_used_bytes_trend)),
            type: "up",
          }}
        />

        <Stat name="Views" value={stats.views.toString()} />

        <Stat
          name="Avg File Size"
          value={formatBytes(Number(stats.average_file_size_bytes))}
        />
      </StatContainer>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <Card className="p-6 lg:col-span-2 flex flex-col">
          <h2 className="text-sm font-medium text-muted-foreground mb-4 tracking-wide uppercase">
            Traffic
          </h2>

          <div className="flex-1 flex items-center justify-center text-muted border border-dashed border-border rounded">
            TRAFFIC CHART
          </div>
        </Card>

        <div className="lg:col-span-1">
          <StorageBreakdown
            total_files={stats.total_files}
            file_types={stats.file_types}
          />
        </div>
      </div>
    </div>
  );
}
