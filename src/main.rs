use std::mem;

use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{FlashWindowEx, FLASHWINFO, FLASHW_ALL},
};

use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize},
    event::{DeviceEvent, MouseButton, StartCause, WindowEvent},
    event_loop::EventLoop,
};

struct LogicalRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
impl LogicalRect {
    fn contains(&self, pos: LogicalPosition<f64>) -> bool {
        pos.x >= self.x
            && pos.x <= self.x + self.width
            && pos.y >= self.y
            && pos.y <= self.y + self.height
    }
}

struct App {
    hwnd: HWND,
    button: LogicalRect,
    window: winit::window::Window,
}
impl App {
    fn new(window: winit::window::Window) -> Self {
        let hwnd = HWND(window.hwnd() as isize);
        let button = LogicalRect {
            x: 150.0,
            y: 130.0,
            width: 100.0,
            height: 40.0,
        };
        Self {
            hwnd,
            button,
            window,
        }
    }

    // FlashWindowEx を呼び出す関数
    fn flash_window(&self) {
        let mut flash_info = FLASHWINFO {
            cbSize: mem::size_of::<FLASHWINFO>() as u32,
            hwnd: self.hwnd,
            dwFlags: FLASHW_ALL,
            uCount: 3,
            dwTimeout: 0,
        };
        unsafe {
            FlashWindowEx(&mut flash_info);
        }
    }
}
// ApplicationHandler トレイトの実装
impl ApplicationHandler for App {
    fn new_events(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, _cause: StartCause) {
        let _ = _event_loop;
        // 必要に応じて初期化コードをここに追加
    }

    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // アプリケーションが再開された際の処理
        println!("アプリケーションが再開されました。");
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, _event: ()) {
        // ユーザーイベントの処理（今回は未使用）
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                // ウィンドウ閉鎖要求時にアプリケーションを終了
                std::process::exit(0);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == winit::event::ElementState::Pressed {
                    if let Some(pos) = self.window.cursor_position() {
                        if self.button.contains(pos) {
                            self.flash_window();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _event: DeviceEvent,
    ) {
        // デバイスイベントの処理（今回は未使用）
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // 待機前の処理（今回は未使用）
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // アプリケーションが一時停止された際の処理
        println!("アプリケーションが一時停止されました。");
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // アプリケーション終了時の処理
        println!("アプリケーションを終了します。");
    }

    fn memory_warning(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // メモリ警告時の処理（今回は未使用）
    }
}

fn main() {
    // イベントループとウィンドウの作成
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("FlashWindowEx Example")
        .with_inner_size(LogicalSize::new(400.0, 300.0))
        .build(&event_loop)
        .unwrap();

    // ウィンドウハンドルの取得
    let hwnd = HWND(window.hwnd() as isize);

    let button = LogicalRect {
        x: 150.0,
        y: 130.0,
        width: 100.0,
        height: 40.0,
    };

    let mut app = App {
        window,
        hwnd,
        button,
    };

    event_loop.run_app(&mut app).unwrap();
}
