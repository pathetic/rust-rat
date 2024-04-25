use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem::{size_of, zeroed};
use winapi::um::shellapi::{NOTIFYICONDATAW, NIM_ADD, NIM_MODIFY, NIM_DELETE, NIF_MESSAGE, NIF_ICON, NIF_TIP, NIF_STATE, NIS_HIDDEN};
use winapi::um::winuser::{WM_APP, LoadIconW};
use std::ptr::null_mut;

const WM_TRAYICON: u32 = WM_APP + 100;

pub struct TrayIcon {
    nid: NOTIFYICONDATAW,
}

impl TrayIcon {
    pub fn new() -> Self {
        let mut nid: NOTIFYICONDATAW = unsafe { zeroed() };
        let hWnd = unsafe { winapi::um::wincon::GetConsoleWindow };

        unsafe {
            nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hWnd();
            nid.uID = 1001;
            nid.uCallbackMessage = WM_TRAYICON;
            nid.hIcon = unsafe { LoadIconW(null_mut(), winapi::um::winuser::IDI_APPLICATION) };
            nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
            nid.dwState = 0;
            nid.dwStateMask = NIS_HIDDEN;
            nid.szTip = [0; 128];
        }

        TrayIcon { nid }
    }

    pub fn show(&mut self) {
        unsafe { winapi::um::shellapi::Shell_NotifyIconW(NIM_ADD, &mut self.nid) };
    }

    pub fn hide(&mut self) {
        unsafe { winapi::um::shellapi::Shell_NotifyIconW(NIM_DELETE, &mut self.nid) };
    }

    fn text_to_tooltip(&mut self, text: &str) -> [u16; 128] {
        let mut trayToolTip = text.to_string();
        let mut trayToolTipInt: [u16; 128] = [0; 128];
        let trayToolTipStrStep: &str = &*trayToolTip;
        let mut trayToolTipStepOS = OsStr::new(trayToolTipStrStep);
        let mut trayToolTipStepUTF16 = trayToolTipStepOS.encode_wide().collect::<Vec<u16>>();
        trayToolTipInt[..trayToolTipStepUTF16.len()].copy_from_slice(&trayToolTipStepUTF16);

        trayToolTipInt
    }

    pub fn set_tooltip(&mut self, tooltip: &str) {
        unsafe { self.nid.szTip = self.text_to_tooltip(tooltip) }
        self.update();
    }

    fn update(&mut self) {
        unsafe { winapi::um::shellapi::Shell_NotifyIconW(NIM_MODIFY, &mut self.nid) };
    }
}