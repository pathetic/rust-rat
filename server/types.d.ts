import React from "react";

export type RATContextType = {
  port: string;
  setPort: (port: string) => void;
  fetchClients: () => void;
  setRunning: (running: boolean) => void;
  running: boolean;
  clientList: Array<RATClient>;
  setSelectedClient: (client: string) => void;
  selectedClient: string;
};

export interface RATProviderProps {
  children: React.ReactNode;
}

export type RATClient = {
  id: string;
  username: string;
  hostname: string;
  ip: string;
  os: string;
  cpu: string;
  ram: string;
  gpus: string[];
  storage: string[];
  displays: number;
  is_elevated: boolean;
  disconnected: boolean;
};

export type ContextMenuType = {
  x: number;
  y: number;
  id: string;
  clientFullName: string;
};

export interface ContextMenuProps {
  x: number;
  y: number;
  id: string;
  onClose: () => void;
  clientFullName: string;
}

export interface ShellCommandType {
  command: string;
  output: React.JSX.Element | string;
}

export interface CommandProps {
  id: string;
  shellStatus: string;
}

export type ProcessType = {
  pid: string;
  name: string;
};
