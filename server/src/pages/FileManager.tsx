import { useEffect, useState, useRef } from "react";
import { useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";

let fileIcon = {
  dir: <i className="ri-folder-fill" style={{ color: "yellow" }}></i>,
  file: <i className="ri-file-fill ri-4x" style={{ color: "white" }}></i>,
  back: <i className="ri-arrow-left-line"></i>,
};

export const FileManager: React.FC = () => {
  const { id } = useParams();

  const [path, setPath] = useState("");
  const [files, setFiles] = useState<Array<string> | null>(null);

  const filesRef = useRef<HTMLDivElement>(null);

  function fileActions(type: string, fileName: string) {
    if (type == "file")
      return (
        <div className="flex flex-row gap-4 mb-4">
          <div
            className="btn btn-outline no-animation"
            onClick={() => manageFile("download_file", fileName)}
          >
            <i className="ri-download-line" style={{ color: "cyan" }}></i>
          </div>
          <div
            className="btn btn-outline no-animation"
            onClick={() => manageFile("remove_file", fileName)}
          >
            <i className="ri-delete-bin-line" style={{ color: "red" }}></i>
          </div>
        </div>
      );
    else if (type == "dir")
      return (
        <div
          className="btn btn-outline no-animation"
          onClick={() => manageFile("remove_dir", fileName)}
        >
          <i className="ri-delete-bin-line" style={{ color: "red" }}></i>
        </div>
      );
    else return <div></div>;
  }

  useEffect(() => {
    fetchFolder("disks");
  }, []);

  async function fetchFolder(folder: string) {
    let ok: Array<string> = await invoke("read_files", {
      id: id,
      run: `${
        folder == "previous"
          ? "previous_dir"
          : folder == "disks"
          ? "available_disks"
          : "view_dir||" + folder
      }`,
    });

    setPath(ok[0]);
    setFiles(ok[1] as unknown as Array<string>);

    if (filesRef.current)
      filesRef.current.scrollIntoView({ behavior: "smooth" });
  }

  async function manageFile(command: string, fileName: string) {
    await invoke("manage_file", {
      id: id,
      run: `${command}||${fileName}`,
    });
  }

  function fileExtension(fileName: string) {
    if (
      fileName.includes(".rar") ||
      fileName.includes(".zip") ||
      fileName.includes(".7z")
    ) {
      return <i className="ri-file-zip-fill ri-4x"></i>;
    } else if (
      fileName.includes(".mp4") ||
      fileName.includes(".mkv") ||
      fileName.includes(".avi")
    ) {
      return <i className="ri-file-video-fill ri-4x"></i>;
    } else if (
      fileName.includes(".mp3") ||
      fileName.includes(".wav") ||
      fileName.includes(".flac")
    ) {
      return <i className="ri-file-music-fill ri-4x"></i>;
    } else if (
      fileName.includes(".jpg") ||
      fileName.includes(".jpeg") ||
      fileName.includes(".png") ||
      fileName.includes(".gif")
    ) {
      return <i className="ri-file-image-fill ri-4x"></i>;
    } else if (fileName.includes(".txt")) {
      return <i className="ri-file-text-fill ri-4x"></i>;
    } else if (fileName.includes(".pdf")) {
      return <i className="ri-file-pdf-fill ri-4x"></i>;
    } else if (fileName.includes(".doc") || fileName.includes(".docx")) {
      return <i className="ri-file-word-fill ri-4x"></i>;
    } else if (fileName.includes(".xls") || fileName.includes(".xlsx")) {
      return <i className="ri-file-excel-fill ri-4x"></i>;
    } else if (fileName.includes(".ppt") || fileName.includes(".pptx")) {
      return <i className="ri-file-ppt-fill ri-4x"></i>;
    } else if (
      fileName.includes(".html") ||
      fileName.includes(".css") ||
      fileName.includes(".js")
    ) {
      return <i className="ri-file-code-fill ri-4x"></i>;
    } else {
      return <i className="ri-file-fill ri-4x" style={{ color: "white" }}></i>;
    }
  }

  return (
    <div className="p-8 overflow-y-auto w-full">
      <p className="text-2xl font-bold pb-2">Current path: {path}</p>

      <div ref={filesRef} className="flex overflow-y-auto pb-2 w-[100%]">
        <div>
          <div
            onClick={() => fetchFolder("previous")}
            className="flex flex-row gap-6 items-center mt-2"
          >
            <div className="w-[350px] flex flex-row gap-4 p-3 text-xl bg-base-300 hover:cursor-pointer rounded-lg">
              <div>{fileIcon["back"]}</div>
              <div>../</div>
            </div>
            <div>{fileActions("back", "")}</div>
          </div>

          {files && files.length > 0 && (
            <>
              {files.map((file) => {
                const parts = file.split("||");
                const fileName = parts[0];
                const fileType = parts[1];

                if (fileType == "dir") {
                  return (
                    <div
                      key={file}
                      className="flex flex-row w-full gap-6 items-center mt-2"
                    >
                      <div
                        onClick={() => fetchFolder(fileName)}
                        className="w-[350px] p-3 flex flex-row gap-4 text-xl bg-base-300 hover:cursor-pointer rounded-lg"
                      >
                        {fileIcon[fileType]}
                        <div
                          className="tooltip break-words"
                          data-tip={fileName}
                        >
                          <span className="flex justify-start	w-[290px] text-ellipsis !overflow-hidden whitespace-nowrap">
                            {fileName}
                          </span>
                        </div>
                      </div>
                      {fileActions(fileType, fileName)}
                    </div>
                  );
                }
              })}
            </>
          )}
        </div>

        <div className="flex flex-wrap content-start mt-4 gap-6 w-full justify-center">
          {files && files.length > 0 && (
            <>
              {files.map((file) => {
                const parts = file.split("||");
                const fileName = parts[0];
                const fileType = parts[1];

                if (fileType == "file") {
                  return (
                    <div
                      key={fileName}
                      className="flex flex-col justify-center items-center align-center w-48 h-48 bg-base-300 rounded-lg"
                    >
                      {fileExtension(fileName)}
                      <div
                        className="tooltip mb-4 break-words"
                        data-tip={fileName}
                      >
                        <span className="w-48 text-ellipsis !overflow-hidden whitespace-nowrap inline-block px-6">
                          {fileName}
                        </span>
                      </div>

                      {fileActions(fileType, fileName)}
                    </div>
                  );
                }
              })}
            </>
          )}
        </div>
      </div>
    </div>
  );
};
