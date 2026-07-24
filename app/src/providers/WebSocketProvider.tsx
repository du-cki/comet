import React, {
  createContext,
  useContext,
  useEffect,
  useRef,
  useState,
  ReactNode,
} from "react";

import { TOKEN_NAME, WS_BASE_URL } from "../utils";
import { Authenticated, User } from "../types";

type ConnectionStatus = "idle" | "connecting" | "connected" | "disconnected";

type WebSocketContextType = {
  user: User | null;
  ws: WebSocket | null;
  status: ConnectionStatus;
  connect: (token: string) => void;
  disconnect: () => void;
  logOut: () => void;
};

const WebSocketContext = createContext<WebSocketContextType | null>(null);

export function WebSocketProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [wsInstance, setWsInstance] = useState<WebSocket | null>(null);
  const [status, setStatus] = useState<ConnectionStatus>("idle");

  const activeSocketRef = useRef<WebSocket | null>(null);

  const connect = (token: string) => {
    if (activeSocketRef.current) return;

    setStatus("connecting");
    const ws = new WebSocket(`${WS_BASE_URL}/pineapple`);
    activeSocketRef.current = ws;

    const handleAuth = (ev: MessageEvent) => {
      const e = JSON.parse(ev.data);
      if (e.type === "Authenticated") {
        setUser((e as Authenticated).data);

        ws?.removeEventListener("message", handleAuth);
      }
    };

    ws.addEventListener("message", handleAuth);

    ws.onopen = () => {
      if (activeSocketRef.current !== ws) return;

      ws.send(JSON.stringify({ token }));

      setStatus("connected");
      console.log("[websocket] connected");
    };

    ws.onclose = () => {
      if (activeSocketRef.current !== ws) return;

      activeSocketRef.current = null;

      setUser(null);
      setWsInstance(null);
      setStatus("disconnected");

      console.log("[websocket] disconnected");
    };

    setWsInstance(ws);
  };

  const disconnect = () => {
    const ws = activeSocketRef.current;
    if (ws) {
      ws.onopen = null;
      ws.onclose = null;
      ws.close();
      activeSocketRef.current = null;
    }

    setUser(null);
    setWsInstance(null);
    setStatus("idle");
  };

  const logOut = () => {
    disconnect();
    localStorage.removeItem(TOKEN_NAME);
  };

  useEffect(() => {
    return () => disconnect();
  }, []);

  return (
    <WebSocketContext.Provider
      value={{ user, ws: wsInstance, status, connect, disconnect, logOut }}
    >
      {children}
    </WebSocketContext.Provider>
  );
}

export function useWebSocket() {
  const context = useContext(WebSocketContext);
  if (!context) {
    throw new Error("useWebSocket must be used within a WebSocketProvider");
  }

  return context;
}
