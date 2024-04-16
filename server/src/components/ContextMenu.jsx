import { useNavigate } from "react-router-dom";

export const ContextMenu = ({ x, y, id, onClose }) => {
  const navigate = useNavigate();

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
