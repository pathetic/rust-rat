import React, { useEffect, useState, useContext } from "react";
import { RATContext } from "../rat/RATContext";
import { ContextMenu } from "../components/ContextMenu";
import { visitWebsiteCmd } from "../rat/RATCommands";

import { ContextMenuType } from "../../types";

import windowsImg from "../assets/732225.png";
import linuxImg from "../assets/pngimg.com - linux_PNG1.png";

export const Clients: React.FC = () => {
  const [selectedClient, setSelectedClient] = useState<string>("");
  const { clientList, fetchClients } = useContext(RATContext)!;
  const [contextMenu, setContextMenu] = useState<ContextMenuType | null>(null);

  const [url , setUrl] = useState<string>("");

  const handleContextMenu = (
    event: React.MouseEvent<HTMLDivElement, MouseEvent>,
    id: string,
    clientFullName: string
  ) => {
    event.preventDefault();
    setSelectedClient(id);
    setContextMenu({
      x: event.pageX,
      y: event.pageY,
      id: id,
      clientFullName,
    });
  };

  const handleVisitWebsite = () => {
    visitWebsiteCmd(String(selectedClient), url);
    (document.getElementById("visit_website_modal") as HTMLDialogElement).close();
    setSelectedClient("");
    setUrl("");
  }

  const handleClose = () => {
    setContextMenu(null);
  };

  useEffect(() => {
    fetchClients();
  }, []);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as HTMLElement;
      if (contextMenu && !target.closest(".context-menu")) {
        setContextMenu(null);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [contextMenu]);

  const fetchGpus = (gpus: string[]) => {
    let gpuString = "";

    gpus.forEach((gpu) => {
      gpuString += `${gpu}\n`;
    });

    return gpuString;
  };

  return (
    <>
      <dialog id="visit_website_modal" className="modal">
        <div className="modal-box">
          <h3 className="font-bold text-lg">Visit Website</h3>
          <div className="form-control mt-6">
            <label className="input input-bordered flex items-center gap-2">
              URL
              <input
                type="text"
                placeholder="https://example.com"
                className="grow"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
              />
            </label>
          </div>
          <div className="modal-action">
            <span onClick={() => handleVisitWebsite()} className="btn">Visit Website</span>
        </div>
        </div>
        
        <form method="dialog" className="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

      <div className="p-8 flex flex-1 flex-col overflow-auto w-full">
        <div className="overflow-y-auto">
          <table className="table">
            <thead>
              <tr>
                <th>ID</th>
                <th>PC Name</th>
                <th>Account Type</th>
                <th>Operating System</th>
                <th>Hardware</th>
              </tr>
            </thead>
            <tbody>
              {clientList && clientList.length > 0 && (
                <>
                  {clientList.map((client) => (
                    <tr
                      key={client.id}
                      onContextMenu={(e) =>
                        handleContextMenu(
                          e,
                          client.id,
                          `${client.username}@${client.hostname}`
                        )
                      }
                    >
                      <th>
                        <p>{client.id}</p>
                      </th>
                      <td>
                        <div className="flex items-center gap-3">
                          <div className="avatar">
                            <div className="mask w-12 h-12">
                              <img
                                src={
                                  client.os.includes("Windows")
                                    ? windowsImg
                                    : linuxImg
                                }
                                alt="OS"
                              />
                            </div>
                          </div>
                          <div>
                            <div className="font-bold">{client.username}</div>
                            <div className="text-sm opacity-50">
                              {client.hostname}
                            </div>
                          </div>
                        </div>
                      </td>
                      <td>{client.is_elevated ? "Admin" : "User"}</td>
                      <td>{client.os}</td>
                      <td>
                        <div className="flex items-center gap-4">
                          <div className="tooltip" data-tip={client.cpu}>
                            <i className="ri-cpu-line ri-2x"></i>
                          </div>

                          <div
                            className="tooltip"
                            style={{ whiteSpace: "pre-line" }}
                            data-tip={fetchGpus(client.gpus)}
                          >
                            <i className="ri-airplay-line ri-2x"></i>
                          </div>

                          <div className="tooltip" data-tip={client.ram}>
                            <i className="ri-ram-2-line ri-2x"></i>
                          </div>

                          <div className="tooltip" data-tip={client.storage}>
                            <i className="ri-hard-drive-3-line ri-2x"></i>
                          </div>
                        </div>
                      </td>
                    </tr>
                  ))}
                </>
              )}
            </tbody>
          </table>
          {contextMenu && (
            <ContextMenu
              x={contextMenu.x}
              y={contextMenu.y}
              id={contextMenu.id}
              onClose={handleClose}
              clientFullName={contextMenu.clientFullName}
            />
          )}
        </div>
      </div>
    </>
  );
};
