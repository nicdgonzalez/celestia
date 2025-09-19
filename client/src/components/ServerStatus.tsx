"use client";

import { useWebSocket } from "./WebSocketProvider";

export function ServerStatus() {
  const { isOnline } = useWebSocket();

  return (
    <p>
      Server is currently: {isOnline ? "Online" : "Offline"}
    </p>
  );
}
