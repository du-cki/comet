import React, {
  createContext,
  useContext,
  useEffect,
  useRef,
  useState,
  ReactNode,
} from "react";

import { WS_BASE_URL } from "../utils";

type ConnectionStatus = "idle" | "connecting" | "connected" | "disconnected";

type WebSocketContextType = {
  ws: WebSocket | null;
  status: ConnectionStatus;
  connect: (token: string) => void;
  disconnect: () => void;
};

const WebSocketContext = createContext<WebSocketContextType | null>(null);

export function WebSocketProvider({ children }: { children: ReactNode }) {
  const [wsInstance, setWsInstance] = useState<WebSocket | null>(null);
  const [status, setStatus] = useState<ConnectionStatus>("idle");

  const activeSocketRef = useRef<WebSocket | null>(null);

  const connect = (token: string) => {
    if (activeSocketRef.current) return;

    setStatus("connecting");
    const ws = new WebSocket(`${WS_BASE_URL}/pineapple`);
    activeSocketRef.current = ws;

    ws.onopen = () => {
      if (activeSocketRef.current !== ws) return;
            ws.send(JSON.stringify({ token }));
      setStatus("connected");
      console.log("[websocket] connected");
    };

    ws.onclose = () => {
      if (activeSocketRef.current !== ws) return;
      activeSocketRef.current = null;
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

    setWsInstance(null);
    setStatus("idle");
  };

  useEffect(() => {
    return () => disconnect();
  }, []);

  return (
    <WebSocketContext.Provider
      value={{ ws: wsInstance, status, connect, disconnect }}
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
