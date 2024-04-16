import { useContext, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { RATContext } from "../rat/RATContext";

export const ContextMenu = ({ x, y, id, onClose, clientFullName }) => {
  const { setSelectedClient } = useContext(RATContext);
  const navigate = useNavigate();

  useEffect(() => {
    setSelectedClient(clientFullName);
    console.log(clientFullName);
  }, [clientFullName]);

  return (
    <div
      className={`fixed context-menu top-[${y}px] left-[${x}px] bg-base-200 border border-white rounded-md list-none flex flex-col text-center`}
      style={{
        top: `${y}px`,
        left: `${x}px`,
      }}
    >
      <div
        className="btn no-animation"
        onClick={() => {
          navigate(`/clients/${id}`);
          onClose();
        }}
      >
        <i className="ri-tools-line ri-2x text-info"></i>
        Manage
      </div>
    </div>
  );
};
