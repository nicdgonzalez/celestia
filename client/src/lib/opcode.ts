export enum Opcode {
  Dispatch = 0,
}

export interface Message {
  op: Opcode;
  d: unknown;
  s?: number;
  t?: string;
}

export type Status = "online" | "offline";

export interface ServerStatusUpdate {
  is_online: boolean;
}

export interface PlayerJoin {
  uuid: string;
  username: string;
  timestamp: string;
}

export interface PlayerLeft {
  uuid: string;
  username: string;
  timestamp: string;
}
