import { useState, useContext } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import toast, { Toaster } from "react-hot-toast";
import { RATContext } from "./rat/RATContext";

export const Server = () => {
  const { port, setPort, setRunning } = useContext(RATContext);

  const navigate = useNavigate();

  async function startServer() {
    let serverMessage = await invoke("start_server", { port });

    if (serverMessage === "true") {
      toast.success("Server started successfully!", {
        className: `bg-secondary text-white text-lg font-bold`,
      });

      setRunning(true);

      const delayPromise = new Promise((resolve, reject) => {
        setTimeout(() => {
          navigate("/home");
          resolve("Redirect complete");
        }, 500);
      });

      toast.promise(
        delayPromise,
        {
          loading: "Redirecting...",
          success: <b>Redirected!</b>,
          error: <b>Could not redirect.</b>,
        },
        {
          className: `bg-secondary text-white text-lg font-bold`,
        }
      );
    } else {
      toast.error("Server failed to start!", {
        className: `bg-secondary text-white text-lg font-bold`,
      });
    }
  }

  return (
    <div className="font-bold text-xl flex justify-center items-center flex-col w-screen h-screen gap-y-4">
      <div className="flex flex-col gap-6 rounded-box bg-base-200 p-6 max-w-md text-center items-center">
        <h1 className="text-2xl font-bold">RAT Server</h1>

        <a target="_blank">
          <img
            src="https://cdn2.iconfinder.com/data/icons/whcompare-isometric-web-hosting-servers/50/value-server-512.png"
            alt="Server Logo"
            width="200px"
          />
        </a>

        <label className="form-control">
          <div className="label">
            <span className="label-text">Port</span>
          </div>

          <input
            value={port}
            onChange={(e) => setPort(e.currentTarget.value)}
            placeholder="Enter a port"
            className="input input-bordered w-full max-w-xs"
          />
        </label>

        <button
          onClick={(e) => {
            e.preventDefault();
            startServer();
          }}
          className="btn btn-outline"
        >
          Start Server
        </button>
      </div>
    </div>
  );
};
