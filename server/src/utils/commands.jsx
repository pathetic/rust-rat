import { invoke } from "@tauri-apps/api/tauri";

export const getOutput = async (command, setCommand, shellStatus, id) => {
  switch (command.toLowerCase()) {
    case "!help":
      return (
        <div>
          {" "}
          Available commands: <br />
          <span className="text-teal-400 ml-3"> !clear</span> - Clear the
          terminal
        </div>
      );
    case "!clear":
      setCommand([]);
      return "";
    default:
      if (shellStatus == "true") {
        const response = await invoke("execute_shell_command", {
          id,
          run: command,
        });
        return <div style={{ whiteSpace: "pre-wrap" }}>{response}</div>;
      } else
        return (
          <div>
            <span className="text-red-500"> Shell is not started. </span>
          </div>
        );
  }
};
