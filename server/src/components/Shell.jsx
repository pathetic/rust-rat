import React, { useState, useRef, useEffect } from "react";
import { keybindings } from "../utils/keybindings";

export const Ascii = () => {
  return (
    <div>
      <pre className="text-left text-primary font-bold mb-2 drop-shadow-xl selection:bg-yellow-900 selection:text-white">
        {`
    
    __________                       __                _________.__           .__  .__   
    \\______   \\ ____   _____   _____/  |_  ____       /   _____/|  |__   ____ |  | |  |  
     |       _// __ \\ /     \\ /  _ \\   __\\/ __ \\      \\_____  \\ |  |  \\_/ __ \\|  | |  |  
     |    |   \\  ___/|  Y Y  (  <_> )  | \\  ___/      /        \\|   Y  \\  ___/|  |_|  |__
     |____|_  /\\___  >__|_|  /\\____/|__|  \\___  >    /_______  /|___|  /\\___  >____/____/
            \\/     \\/      \\/                 \\/             \\/      \\/     \\/              
              `}
      </pre>
    </div>
  );
};

export const Header = () => {
  return (
    <div className="font-mono text-left ml-10 mb-5  selection:bg-yellow-900">
      Welcome to Remote Shell!
      <br />
      Type <span className="text-primary">!help</span> to get a list of
      available client-side commands. <br />
    </div>
  );
};

export const Command = ({ id, shellStatus }) => {
  const [command, setCommand] = useState([]);
  const [currentCommand, setCurrentCommand] = useState("");
  const [upArrowKeyPressed, setUpArrowKeyPressed] = useState(0);
  const inputRef = useRef(null);

  useEffect(() => {
    inputRef.current.scrollIntoView({
      behavior: "smooth",
      block: "nearest",
      inline: "start",
    });
    inputRef.current.focus();
  }, [command]);

  useEffect(() => {
    const handleClick = (event) => {
      if (inputRef.current && !inputRef.current.contains(event.target)) {
        // inputRef.current.scrollIntoView({
        //   behavior: "smooth",
        //   block: "nearest",
        //   inline: "start",
        // });
        inputRef.current.focus();
      }
    };

    document.addEventListener("click", handleClick);

    return () => {
      document.removeEventListener("click", handleClick);
    };
  }, []);

  return (
    <div>
      {command.map((item, index) => (
        <div key={index}>
          <div className="flex flex-row mb-0.5">
            <div className="text-accent font-bold ml-10  selection:bg-yellow-900">
              remote@shell~$
            </div>
            <div className="ml-2 font-mono selection:bg-yellow-900">
              {item.command}
            </div>
          </div>
          <div
            className="font-mono text-left ml-16 mb-3 selection:bg-yellow-900 mr-10"
            id="output-content"
          >
            {item.output}
          </div>
        </div>
      ))}
      <div className="flex flex-row">
        <div className="text-accent font-bold ml-10 selection:bg-yellow-900">
          remote@shell~$
        </div>
        <input
          className="bg-transparent outline-none border-none font-mono ml-2  w-2/3"
          type="text"
          ref={inputRef}
          value={currentCommand}
          autoFocus={true}
          onChange={(e) => setCurrentCommand(e.target.value)}
          onKeyDown={(e) => {
            keybindings(
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
            );
          }}
        />
      </div>
    </div>
  );
};
