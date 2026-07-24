import React, { useEffect, useState } from "react";

import { api } from "../client";
import { useWebSocket } from "../providers/WebSocketProvider";
import { debounce, formatBytes } from "../utils";

import { LoaderCircle } from "lucide-react";

import type { DashboardStats } from "../types";

import Card from "../components/common/Card";
import { Stat, StatContainer } from "../components/Stats";
import { SettingItem } from "../components/SettingItem";

function Settings() {
  const [isLoading, setIsLoading] = useState(true);

  const [maintenanceMode, setMaintenanceMode] = useState(false);
  const [publicSignUps, setPublicSignUps] = useState(false);
  const [enforceExtensions, setEnforceExtensions] = useState(false);

  const [limitUploadSize, setLimitUploadSize] = useState(false);
  const [maxUploadSize, setMaxUploadSize] = useState(50);

  const [fileNameLength, setFileNameLength] = useState(8);

  const [require2FA, setRequire2FA] = useState(false);

  useEffect(() => {
    const fetchSettings = async () => {
      const data = await api.getSettings();

      setMaintenanceMode(data.maintenance_mode);
      setPublicSignUps(data.allow_public_signups);
      setEnforceExtensions(data.enforce_file_extensions);
      setRequire2FA(data.require_2fa);

      setFileNameLength(data.file_name_length);

      if (data.max_upload_size_mb === null || data.max_upload_size_mb <= 0) {
        setLimitUploadSize(false);
        setMaxUploadSize(50);
      } else {
        setLimitUploadSize(true);
        setMaxUploadSize(data.max_upload_size_mb);
      }

      setIsLoading(false);
    };

    fetchSettings();
  }, []);

  const debouncedApiUpdate = debounce((key: string, value: any) => {
    api
      .updateSetting(key, value)
      .catch((err) => console.error(`Failed to update ${key}:`, err));
  }, 800);

  const handleFileNameLengthChange = (val: number) => {
    setFileNameLength(val);
    debouncedApiUpdate("file_name_length", val);
  };

  const handleMaxUploadSizeChange = (val: number) => {
    setMaxUploadSize(val);
    if (limitUploadSize) {
      debouncedApiUpdate("max_upload_size_mb", val);
    }
  };

  const handleLimitToggle = (enabled: boolean) => {
    setLimitUploadSize(enabled);

    api.updateSetting("max_upload_size_mb", enabled ? maxUploadSize : -1);
  };

  const handleToggle = async (
    key: string,
    newValue: boolean,
    setterFn: React.Dispatch<React.SetStateAction<boolean>>,
  ) => {
    setterFn(newValue);

    try {
      await api.updateSetting(key, newValue);
    } catch (error) {
      setterFn(!newValue);
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center">
        <LoaderCircle
          className="animate-spin text-muted-foreground"
          size={32}
        />
      </div>
    );
  }

  return (
    <>
      <Card label="Global Settings" className="space-y-4 w-full max-w-2xl">
        <SettingItem
          label="Maintenance Mode"
          description="Sets the app on read-only mode."
          checked={maintenanceMode}
          onToggle={(val) =>
            handleToggle("maintenance_mode", val, setMaintenanceMode)
          }
        />

        <SettingItem
          label="Allow Public Sign-Ups"
          description="Auto-Approve any new accounts."
          checked={publicSignUps}
          onToggle={(val) =>
            handleToggle("allow_public_signups", val, setPublicSignUps)
          }
        />

        <SettingItem
          label="Enforce File Extensions"
          description="Mandates file extensions on all requests."
          checked={enforceExtensions}
          onToggle={(val) =>
            handleToggle("enforce_file_extensions", val, setEnforceExtensions)
          }
        />

        <SettingItem
          label="File Name Length"
          description="The character length of randomly generated CDN file names."
          value={fileNameLength}
          onValueChange={handleFileNameLengthChange}
          unit="chars"
        />

        <SettingItem
          label="Max Upload Size"
          description="Restrict the maximum file size for a single upload."
          checked={limitUploadSize}
          onToggle={handleLimitToggle}
          value={maxUploadSize}
          onValueChange={handleMaxUploadSizeChange}
          unit="MB"
        />
      </Card>

      <Card label="Privacy & Security" className="w-full max-w-2xl">
        <SettingItem
          label="Require 2FA"
          description="Enforce Two-Factor Authentication for all users."
          checked={require2FA}
          onToggle={(val) => handleToggle("require_2fa", val, setRequire2FA)}
        />
      </Card>
    </>
  );
}

export default function Admin() {
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

    ws.send(JSON.stringify({ action: "GetAdminStats" }));

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

      <Settings />
    </div>
  );
}
