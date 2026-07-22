import React, { useEffect } from "react";

import { Outlet, useNavigate } from "react-router-dom";
import { useWebSocket } from "./WebSocketProvider";

import { LoaderCircle } from "lucide-react";
import { NavBar } from "../components/NavBar";

export default function ProtectedLayout() {
  const { ws, status, connect } = useWebSocket();
  const navigate = useNavigate();

  useEffect(() => {
    const token = localStorage.getItem("auth_token");
    if (!token) {
      navigate("/", { replace: true });
      return;
    }

    if (status === "idle") {
      connect(token);
    }
  }, [status, connect, navigate]);

  if (status === "idle" || !ws) {
    return (
      <div className="flex min-h-svh items-center justify-center">
        <LoaderCircle
          className="animate-spin text-muted-foreground"
          size={32}
        />
      </div>
    );
  }

  return (
    <div className="flex h-screen w-full bg-black text-white overflow-hidden">
      <NavBar />

      <main className="flex-1 overflow-y-auto p-4 md:p-8 pl-20 md:pl-24">
        <div className="max-w-7xl mx-auto">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
