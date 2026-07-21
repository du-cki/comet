import React, { useEffect, useState } from "react";

import { useWebSocket } from "../providers/WebSocketProvider";
import type { DashboardStats } from "../types";

import { formatBytes } from "../utils";

import { Stat, StatContainer } from "../components/Stats";

export default function Dashboard() {
  const [stats, setStats] = useState<DashboardStats["data"]>({
    total_files: 0,
    total_files_trend: 0,
    storage_used_bytes: 0,
    storage_used_bytes_trend: 0,
    views: 0,
    average_file_size_bytes: 0,
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
    <div className="w-full flex justify-center">
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
    </div>
  );
}
