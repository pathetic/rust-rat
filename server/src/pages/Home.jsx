import { useContext } from "react";
import { RATContext } from "../rat/RATContext";

export const Home = () => {
  const { running, clientList } = useContext(RATContext);

  return (
    <div className="p-8 flex flex-1 flex-col overflow-auto w-full">
      <div className="stats shadow border border-secondary">
        <div className="stat">
          <div className="stat-figure">
            <i className="ri-user-fill ri-2x"></i>
          </div>
          <div className="stat-title">Online Clients</div>
          <div className="stat-value">{clientList.length}</div>
        </div>

        <div className="stat">
          <div className="stat-figure ">
            <i className="ri-server-fill ri-2x"></i>
          </div>
          <div className="stat-title ">Server Status</div>
          <div className="stat-value ">{running ? "Online" : "Offline"}</div>
        </div>
      </div>
    </div>
  );
};
