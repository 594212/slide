use std::collections::VecDeque;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::winuser::{
    GetWindowTextLengthW, GetWindowTextW, IsWindowVisible, SetWindowPos, SWP_NOACTIVATE,
    SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, WINDOWINFO,
};

use crate::{Stop, Window};

pub struct Reel(VecDeque<Stop>);
impl Reel {
    pub fn to_vec_deque(&self) -> &VecDeque<Stop> {
        &self.0
    }

    pub fn set_on_front(&mut self, pos: usize) {
        self.helper_on_front(pos);
        self.0.remove(pos).map(|el| self.0.push_front(el));
    }

    fn helper_on_front(&self, pos: usize) {
        for i in 0..pos {
            unsafe {
                SetWindowPos(
                    self.0[i].hwnd,
                    self.0[pos].hwnd,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE ^ SWP_NOSIZE ^ SWP_SHOWWINDOW ^ SWP_NOACTIVATE,
                )
            };
        }
    }

    pub fn new<T>(queue: T) -> Self
    where
        T: Into<VecDeque<Stop>>,
    {
        return Reel(queue.into());
    }

    pub fn next(&mut self) -> &Stop {
        self.helper_on_front(1);
        self.0.swap(0, 1);
        &self.0[0]
    }

    pub fn prev(&mut self) -> &Stop {
        self.helper_on_front(self.0.len());
        self.0.pop_back().map(|e| self.0.push_front(e));
        &self.0[0]
    }

    pub fn debug_window(window: &Window) {
        unsafe {
            let hwnd = window.clone();
            let window_info: WINDOWINFO = std::mem::zeroed();
            let len = GetWindowTextLengthW(hwnd) as usize;

            let mut buffer: Vec<u16> = vec![0; len + 1];
            GetWindowTextW(hwnd, buffer.as_mut_ptr(), (len + 1) as i32);

            dbg!(
                hwnd,
                OsString::from_wide(&buffer[..len]),
                IsWindowVisible(hwnd),
                window_info.cxWindowBorders,
                window_info.cyWindowBorders,
            );
            println!("-------------------");
        }
    }

    pub fn debug(&self) {
        self.0.iter().for_each(|e| Self::debug_window(&e.hwnd));
    }
}
