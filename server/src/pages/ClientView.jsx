import { useEffect, useState, useRef } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

export const ClientView = () => {
  const { id } = useParams();
  const [client, setClient] = useState({});
  const [loaded, setLoaded] = useState(false);
  const [screenshot, setScreenshot] = useState(null);

  const navigate = useNavigate();

  async function fetchClient() {
    let ok = await invoke("fetch_client", { id });
    setClient(ok);
    setLoaded(true);
  }

  async function waitScreenshot() {
    const unlisten = listen("client_screenshot", (event) => {
      setScreenshot(event.payload);
    });
  }

  async function takeScreenshot(display) {
    let ok = await invoke("take_screenshot", {
      id,
      display: parseInt(display),
    });
  }

  async function handleSystem(cmd) {
    let ok = await invoke("handle_system_command", { id, run: cmd });
  }

  useEffect(() => {
    waitScreenshot();
  }, []);

  useEffect(() => {
    if (client.disconnected) {
      navigate("/clients");
    }
  }, [client]);

  const fetchVec = (vec) => {
    let vecString = "\n";

    vec.forEach((v) => {
      vecString += `${v}\n`;
    });

    return vecString;
  };

  useEffect(() => {
    fetchClient();
  }, []);

  return (
    <div className="client p-8 flex flex-1 flex-col overflow-auto w-full items-center">
      <div className="flex flex-row gap-8">
        {loaded ? (
          <div className="card bg-base-100 !min-w-[350px] shadow-xl border border-white">
            <div className="card-body" style={{ whiteSpace: "pre-line" }}>
              <h2 className="card-title">User Information</h2>
              <a>OS: {client.os}</a>
              <a>Username: {client.username}</a>
              <a>Hostname: {client.hostname}</a>
              <a>Account Type: {client.is_elevated ? "Admin" : "User"}</a>
              <a>IP Address: {client.ip}</a>
              <a>CPU: {client.cpu}</a>
              <a>RAM: {client.ram}</a>
              <a>GPUs: {fetchVec(client.gpus)}</a>
              <a>Drives: {fetchVec(client.storage)}</a>

              <div className="card bg-base-100 shadow-xl border border-white mt-4">
                <div className="card-body">
                  <h2 className="card-title">Remote Management</h2>
                  <a
                    onClick={() => navigate(`/clients/${id}/files`)}
                    className="btn btn-active"
                  >
                    File Manager
                  </a>
                  <a
                    onClick={() => navigate(`/clients/${id}/shell`)}
                    className="btn btn-active"
                  >
                    Remote Shell
                  </a>
                  <a
                    onClick={() => navigate(`/clients/${id}/process`)}
                    className="btn btn-active"
                  >
                    Process List
                  </a>
                </div>
              </div>

              <div className="card bg-base-100 shadow-xl border border-white mt-4">
                <div className="card-body">
                  <h2 className="card-title">System Actions</h2>
                  <a
                    onClick={() => handleSystem("shutdown")}
                    className="btn btn-error"
                  >
                    Shutdown
                  </a>
                  <a
                    onClick={() => handleSystem("restart")}
                    className="btn btn-warning"
                  >
                    Restart
                  </a>
                  <a
                    onClick={() => handleSystem("logout")}
                    className="btn btn-info"
                  >
                    Log Out
                  </a>
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div className="skeleton card bg-base-100 !min-w-[350px] shadow-xl border border-white"></div>
        )}

        <div className="card bg-base-100 shadow-xl border border-white">
          <h2 className="card-title pl-12 pt-8">User Desktop</h2>
          <figure className="px-10 pt-10">
            {screenshot ? (
              <img
                src={`data:image/png;base64,${screenshot}`}
                className="rounded-xl w-[100vh] aspect-video"
              />
            ) : (
              <div className="skeleton rounded-xl w-full aspect-video min-w-[100vh]"></div>
            )}
          </figure>
          <div className="card-body items-center text-center">
            <div className="">
              {[...Array(client.displays).keys()].map((index) => (
                <a
                  key={index}
                  onClick={() => takeScreenshot(index)}
                  className="btn btn-active"
                >
                  Screenshot Display {index}
                </a>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
