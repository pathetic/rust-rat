import { getOutput } from "./commands";

export const keybindings = async (
  e,
  inputRef,
  setCommand,
  setUpArrowKeyPressed,
  setCurrentCommand,
  currentCommand,
  command,
  id,
  shellStatus,
  upArrowKeyPressed
) => {
  if (e.key === "Enter") {
    const output = await getOutput(currentCommand, setCommand, shellStatus, id);
    setCommand((prevCommand) => [
      ...prevCommand,
      {
        command: currentCommand.toLowerCase(),
        output,
      },
    ]);
    setCurrentCommand("");
    setUpArrowKeyPressed(0);
  } else if (e.keyCode === 38) {
    if (command.length > 0) {
      setUpArrowKeyPressed(upArrowKeyPressed + 1);
      if (command.length - upArrowKeyPressed - 1 >= 0) {
        setCurrentCommand(
          command[command.length - upArrowKeyPressed - 1].command
        );
      }
    }
  } else if (e.key === "Tab") {
    e.preventDefault();
    if (currentCommand.toLowerCase().startsWith("!c")) {
      setCurrentCommand("!clear");
    } else if (currentCommand.toLowerCase().startsWith("!h")) {
      setCurrentCommand("!help");
    }
  }
};
