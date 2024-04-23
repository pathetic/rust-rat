import React from "react";

export type RATState = {
  running: boolean;
  port: string;
};

export type RATContextType = {
  port: string;
  setPort: (port: string) => void;
  fetchClients: () => void;
  setRunning: (running: boolean) => void;
  running: boolean;
  clientList: Array<RATClient>;
  setSelectedClient: (client: string) => void;
  selectedClient: string;
  setNotificationClient: (notificationClient: boolean) => void;
  notificationClient: boolean;
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

export type MenuOptionType = {
  label: string;
  icon: React.ReactNode;
  navigate?: boolean;
  path?: string;
  options?: MenuOptionType[];
};

interface SubMenuProps {
  items: MenuOptionType[];
  top: number;
  left: number;
  id: string;
  navigate: (string) => void;
  onClose: () => void;
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

export type FileType = {
  file_type: string;
  name: string;
};
