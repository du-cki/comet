import React, { useEffect, useState } from "react";

import { api } from "../client";

import { LoaderCircle } from "lucide-react";

import Card from "../components/common/Card";
import Toggle from "../components/common/Toggle";
import { debounce } from "../utils";

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

export default function AdminSettings() {
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
    <div className="space-y-6">
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
    </div>
  );
}
