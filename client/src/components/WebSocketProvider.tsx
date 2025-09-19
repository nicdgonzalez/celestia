"use client";

import type { PropsWithChildren } from "react";
import { createContext, useContext, useEffect, useState } from "react";

// TODO: Move to separate module. ---------------------------------------------

enum Opcode {
  Dispatch = 0,
}

interface GatewayEvent {
  op: Opcode;
  d: unknown;
  s?: number;
  t?: string;
}

type Status = "online" | "offline";

interface ServerStatusUpdate {
  status: Status;
}

interface PlayerJoin {
  name: string;
}

interface PlayerLeft {
  name: string;
}

interface Player {
  name: string;
}

// ----------------------------------------------------------------------------

interface WebSocketData {
  isOnline: boolean;
  currentPlayers: Player[];
}

const WebSocketContext = createContext<WebSocketData>({
  isOnline: false,
  currentPlayers: [],
});

interface WebSocketProviderProps extends PropsWithChildren {
  url: string;
}

export function WebSocketProvider({ children, url }: WebSocketProviderProps) {
  const [isOnline, setIsOnline] = useState<boolean>(false);
  const [currentPlayers, setCurrentPlayers] = useState<Player[]>([]);

  useEffect(() => {
    const socket = new WebSocket(url);

    socket.addEventListener("message", (event) => {
      const message = JSON.parse(event.data);
      handleMessage(message);
    });

    return () => void socket.close();
  });

  const handleMessage = (message: GatewayEvent) => {
    if (message.op !== Opcode.Dispatch) {
      return;
    }

    console.assert(typeof message.t !== "undefined");

    switch (message.t) {
      case "SERVER_STATUS_UPDATE": {
        const data = message.d as ServerStatusUpdate;
        setIsOnline(data.status === "online");
        break;
      }
      case "PLAYER_JOIN": {
        const data = message.d as PlayerJoin;
        setCurrentPlayers((previousPlayers) => [
          ...previousPlayers,
          { name: data.name },
        ]);
        break;
      }
      case "PLAYER_LEFT": {
        const data = message.d as PlayerLeft;
        setCurrentPlayers((previousPlayers) =>
          previousPlayers.filter((p) => p.name !== data.name)
        );
        break;
      }
      default:
        break;
    }
  };

  return (
    <WebSocketContext.Provider
      value={{ isOnline, currentPlayers }}
    >
      {children}
    </WebSocketContext.Provider>
  );
}

export function useWebSocket() {
  return useContext(WebSocketContext);
}
