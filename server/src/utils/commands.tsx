import { invoke } from "@tauri-apps/api/tauri";
import { ShellCommandType } from "../../types";

export const getOutput = async (
  command: string,
  setCommand: React.Dispatch<React.SetStateAction<ShellCommandType[]>>,
  shellStatus: string,
  id: string
): Promise<JSX.Element | string> => {
  switch (command.toLowerCase()) {
    case "!help":
      return (
        <div>
          {" "}
          Available commands: <br />
          <span className="text-primary ml-3"> !clear</span> - Clear the
          terminal
        </div>
      );
    case "!clear":
      setCommand([]);
      return "";
    default:
      if (shellStatus == "true") {
        const response: string = await invoke("execute_shell_command", {
          id,
          run: command,
        });
        return <div style={{ whiteSpace: "pre-wrap" }}>{response}</div>;
      } else
        return (
          <div>
            <span className="text-error"> Shell is not started. </span>
          </div>
        );
  }
};
