import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import { Ascii, Command, Header } from "../components/Shell";
import { invoke } from "@tauri-apps/api/tauri";

export const RemoteShell = () => {
  const { id } = useParams();
  const [shellStatus, setShellStatus] = useState("false");

  async function manage_shell(id, run) {
    const ok = await invoke("manage_shell", { id, run });
    setShellStatus(ok);
  }

  useEffect(() => {
    manage_shell(id, "status");
  }, []);

  return (
    <div className="p-8 flex flex-1 flex-col overflow-auto w-full">
      <div className="flex flex-row gap-6 pb-4">
        <a
          onClick={() => manage_shell(id, "start")}
          className={`btn btn-success ${
            shellStatus == "true" ? "btn-disabled" : ""
          }`}
        >
          Start Shell
        </a>
        <a
          onClick={() => manage_shell(id, "stop")}
          className={`btn btn-error ${
            shellStatus == "true" ? "" : "btn-disabled"
          }`}
        >
          Stop Shell
        </a>
      </div>
      <div className="bg-neutral-800 text-slate-300 w-full h-screen overflow-x-hidden">
        <Ascii />
        <Header />
        <Command id={id} shellStatus={shellStatus} />
      </div>
    </div>
  );
};
