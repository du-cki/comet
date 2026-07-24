import React, { SyntheticEvent, useEffect, useState } from "react";

import { useWebSocket } from "../providers/WebSocketProvider";
import { BASE_URL, getAbsoluteUrl, useTransitionNavigate } from "../utils";
import { api } from "../client";

import { Key, Copy, RefreshCw, Check } from "lucide-react";
import { SiSharex } from "@icons-pack/react-simple-icons";

import Card from "../components/common/Card";
import Input from "../components/common/Input";
import Button from "../components/common/Button";
import { SettingLabel } from "../components/SettingItem";

export default function Settings() {
  const navigate = useTransitionNavigate();

  const [isLoading, setIsLoading] = useState(false);
  const [isCopied, setIsCopied] = useState(false);

  const [apiKey, setApiKey] = useState<string | null>(null);

  const { user, logOut } = useWebSocket();

  useEffect(() => {
    if (!user) return;
    setApiKey(user.api_key);
  }, [user]);

  const handlePasswordReset = async (e: SyntheticEvent<HTMLFormElement>) => {
    e.preventDefault();
    try {
      setIsLoading(true);
      const formData = new FormData(e.currentTarget);
      const formValues = Object.fromEntries(formData.entries());

      const { currentP, newP } = formValues;
      if (!currentP || !newP) return;

      await api.resetPassword(currentP as string, newP as string);
      logOut();
      navigate("/");
    } catch (e) {
      alert("couldn't change your password!");
    } finally {
      setIsLoading(false);
    }
  };

  const handleRegenerateApiKey = async () => {
    const req = await api.resetKey();

    if (req.ok) {
      const { api_key } = await req.json();
      setApiKey(api_key);
    }
  };

  const handleConfigDownload = async (toolName: string) => {
    if (!apiKey) return;

    try {
      const toolId = toolName.toLowerCase();

      const params = new URLSearchParams({
        tool: toolId,
        api_key: apiKey,
        domain: getAbsoluteUrl(BASE_URL),
      });

      const downloadUrl = getAbsoluteUrl(`${BASE_URL}/config?${params}`);
      window.open(downloadUrl, "_blank");
    } catch {}
  };

  const integrations = [
    {
      name: "ShareX",
      icon: <SiSharex className="h-4 w-4" />,
    },
  ];

  return (
    <div className="space-y-6 w-full max-w-2xl">
      <Card label="Account Management" className="space-y-8 p-6">
        <SettingLabel
          label="Change Password"
          description="Update your account access credentials."
        >
          <form onSubmit={handlePasswordReset} className="space-y-4 max-w-md">
            <Input
              name="currentP"
              type="password"
              placeholder="Current Password"
              disabled={isLoading}
              required
            />

            <Input
              name="newP"
              type="password"
              placeholder="New Password"
              disabled={isLoading}
              required
            />

            <Button
              variant="secondary"
              type="submit"
              className="w-fit text-sm"
              isLoading={isLoading}
            >
              Update Password
            </Button>
          </form>
        </SettingLabel>

        <hr className="border-border" />

        <section>
          <SettingLabel
            label="API Key"
            description="Used to authenticate external tools like ShareX or custom scripts."
          >
            <div className="flex items-center gap-3">
              <div className="relative flex-1">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <Key className="h-4 w-4 text-muted-foreground" />
                </div>

                <Input
                  type="password"
                  readOnly
                  value={apiKey || ""}
                  disabled={!apiKey}
                  className="pl-10 font-mono w-full"
                />
              </div>

              <Button
                size="icon"
                variant="bordered"
                onClick={() => {
                  if (apiKey) {
                    navigator.clipboard.writeText(apiKey);

                    setIsCopied(true);
                    setTimeout(() => setIsCopied(false), 2000);
                  }
                }}
                disabled={!apiKey}
                title="Copy API Key"
              >
                {isCopied ? (
                  <Check className="h-4 w-4 text-green-500" />
                ) : (
                  <Copy className="h-4 w-4" />
                )}
              </Button>

              <Button
                size="icon"
                variant="bordered"
                onClick={handleRegenerateApiKey}
                disabled={!apiKey}
                title="Regenerate API Key"
                className="hover:bg-destructive/10 hover:text-destructive hover:border-destructive/30"
              >
                <RefreshCw className="h-4 w-4" />
              </Button>
            </div>
          </SettingLabel>
        </section>
      </Card>

      <Card label="Integrations" className="p-6">
        {integrations.map(({ name, icon }) => (
          <Button
            key={name}
            variant="bordered"
            className="w-fit"
            onClick={() => handleConfigDownload(name)}
          >
            {icon}
            {name}
          </Button>
        ))}
      </Card>
    </div>
  );
}
