use eframe::egui;
use std::env;

use log::error;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "シンプルなボタン",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(cc)))),
    );
}

struct MyApp {
    window: eframe::Window,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_fonts(egui::FontDefinitions::default());
        Self {
            // 初期状態の設定
        }
    }

    fn flash_taskbar(&self) {
        unsafe {
            let h = match self.window.handle.hwnd() {
                Some(h) => h,
                None => {
                    error!("ウィンドウハンドルが取得できませんでした。");
                    return;
                }
            };
            let hwnd = HWND(h as *mut _);
            let mut flash_info = FLASHWINFO {
                cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
                hwnd,
                dwFlags: FLASHW_ALL,
                uCount: 3,
                dwTimeout: 0,
            };
            FlashWindowEx(&mut flash_info);
        }
    }
}
