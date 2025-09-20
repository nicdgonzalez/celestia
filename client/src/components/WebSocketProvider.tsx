"use client";

import type { PropsWithChildren } from "react";
import { createContext, useContext, useEffect, useState } from "react";
import type {
  Message,
  PlayerJoin,
  PlayerLeft,
  ServerStatusUpdate,
} from "@/lib/opcode";
import { Opcode } from "@/lib/opcode";

export interface Player {
  uuid: string;
  username: string;
}

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

  const handleMessage = (message: Message) => {
    if (message.op !== Opcode.Dispatch) {
      return;
    }

    console.assert(typeof message.t !== "undefined");

    switch (message.t) {
      case "SERVER_STATUS_UPDATE": {
        const data = message.d as ServerStatusUpdate;
        setIsOnline(data.is_online);
        break;
      }
      case "PLAYER_JOIN": {
        const data = message.d as PlayerJoin;
        setCurrentPlayers((previousPlayers) => [
          ...previousPlayers,
          { uuid: data.uuid, username: data.username },
        ]);
        break;
      }
      case "PLAYER_LEFT": {
        const data = message.d as PlayerLeft;
        setCurrentPlayers((previousPlayers) =>
          previousPlayers.filter((p) => p.uuid !== data.uuid)
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

/**
 * Use the data provided by the `WebSocketProvider`.
 */
export function useWebSocket() {
  return useContext(WebSocketContext);
}
