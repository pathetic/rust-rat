import { useEffect, useState, useRef } from "react";
import { useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";
import { ProcessType } from "../../types";

export const ProcessList: React.FC = () => {
  const { id } = useParams();
  const [processes, setProcesses] = useState<ProcessType[] | null>(null);
  const [processFilter, setProcessFilter] = useState("");

  async function fetchProcessList() {
    let ok: string = await invoke("process_list", { id: id });

    const entries = ok.replace("processes||", "").split(",");

    setProcesses(
      entries.map((entry) => {
        const parts = entry.split("||");
        return {
          pid: parts[0],
          name: parts[1],
        };
      })
    );
  }

  useEffect(() => {
    fetchProcessList();
  }, []);

  return (
    <div className="p-8 flex flex-1 flex-col overflow-auto w-full">
      <div className="flex flex-row gap-4 pb-4 w-[30%]">
        <a className="btn btn-active" onClick={fetchProcessList}>
          <i className="ri-refresh-fill ri-2x"></i>
          Refresh
        </a>
        <div>
          <label className="input input-bordered flex items-center gap-2 w-[100%]">
            Process Name
            <input
              value={processFilter}
              onChange={(e) => setProcessFilter(e.target.value)}
              type="text"
              className="grow"
              placeholder="Process name"
            />
          </label>
        </div>
      </div>
      <div className="overflow-x-auto">
        <table className="table table-zebra">
          <thead>
            <tr>
              <th>PID</th>
              <th>Process Name</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {processes && processes.length > 0 && (
              <>
                {processes.map((process, index) => {
                  if (
                    processFilter &&
                    !process.name
                      .toLowerCase()
                      .includes(processFilter.toLowerCase())
                  )
                    return null;
                  return (
                    <tr key={index}>
                      <td>{process.pid}</td>
                      <td>{process.name}</td>
                      <td>
                        <button
                          className="btn btn-active no-animation"
                          onClick={() =>
                            invoke("kill_process", {
                              id: id,
                              pid: parseInt(process.pid),
                            })
                          }
                        >
                          <i
                            className="ri-close-line"
                            style={{ color: "red" }}
                          ></i>
                          Kill Process
                        </button>
                      </td>
                    </tr>
                  );
                })}
              </>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};
