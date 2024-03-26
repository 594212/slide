use std::io;

use gui::Front;
use iced::widget::tooltip::Position;
use iced::Application;
use iced::Size;
use reel::Reel;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::HWND;
use winapi::um::dwmapi::DwmGetWindowAttribute;
use winapi::um::dwmapi::DWMWA_CLOAKED;
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::GetWindowInfo;
use winapi::um::winuser::GetWindowTextLengthW;
use winapi::um::winuser::GetWindowTextW;
use winapi::um::winuser::WINDOWINFO;
use winapi::um::winuser::{EnumWindows, IsWindowVisible};
mod gui;
mod reel;

fn main() {
    let mut windows: std::collections::VecDeque<Stop> = get_all_windows().into();
    let app = (0x0000000000090416 as HANDLE) as HWND;
    if let Some(pos) = windows.iter().position(|x| x.hwnd == app) {
        windows.remove(pos);
    }

    unsafe { winapi::um::winuser::SetForegroundWindow(app.clone()) };

    let reel: Reel = Reel::new(windows);
    let mut setting = iced::Settings::with_flags(reel);
    setting.window.size = Size::new(256.0, 768.0);
    setting.window.position = iced::window::Position::Specific(iced::Point::new(10.0, 10.0));
    setting.window.transparent = true;
    Front::run(setting);
}

fn demo(reel: &mut Reel) {
    loop {
        reel.debug();
        println!("Please input number window that you wanna set in front:");
        let mut input = String::new();
        let result = io::stdin()
            .read_line(&mut input)
            .map_err(|_| "Failed to read input") // Convert io::Error to a custom error message
            .and_then(|_| {
                input
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse input as a positive number")
            });

        // Match on the combined result
        match result {
            Ok(pos) => reel.set_on_front(pos),
            Err(e) => println!("{}", e), // Print a single error message for any error
        }
    }
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let windows = &mut *(lparam as *mut Vec<Stop>);

    let mut is_cloaked: BOOL = 0;
    DwmGetWindowAttribute(
        hwnd,
        DWMWA_CLOAKED,
        &mut is_cloaked as *mut _ as *mut c_void,
        std::mem::size_of::<BOOL>() as u32,
    );

    let mut window_info: WINDOWINFO = std::mem::zeroed();
    window_info.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
    GetWindowInfo(hwnd, &mut window_info);

    let len = GetWindowTextLengthW(hwnd) as usize;

    if IsWindowVisible(hwnd) != TRUE //1
        || len ==0
        || is_cloaked != FALSE //0
        || window_info.cxWindowBorders == 0
        || window_info.cyWindowBorders == 0
    {
        return TRUE;
    }
    let stop = Stop::new(WindowDto(len, hwnd));

    windows.extend(stop);

    return TRUE;
}

fn get_all_windows() -> Vec<Stop> {
    let mut windows: Vec<Stop> = Vec::new();
    let windows_ptr = &mut windows as *mut _ as LPARAM;

    unsafe {
        EnumWindows(Some(enum_windows_callback), windows_ptr);
    }

    windows
}
type Window = HWND;
struct WindowDto(usize, Window);

pub struct Stop {
    hwnd: Window,
    title: String,
}

impl Stop {
    fn new(dto: WindowDto) -> Option<Self> {
        let title: std::ffi::OsString;

        unsafe {
            let mut buffer: Vec<u16> = vec![0; dto.0 + 1];
            GetWindowTextW(dto.1, buffer.as_mut_ptr(), (dto.0 + 1) as i32);

            title = std::os::windows::ffi::OsStringExt::from_wide(&buffer[..dto.0]);
        }

        Some(Self {
            hwnd: dto.1,
            title: title.into_string().ok()?,
        })
    }

    pub fn hwnd(&self) -> &Window {
        &self.hwnd
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
