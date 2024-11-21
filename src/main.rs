use env_logger::Env;
use log::{error, info};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(Window::default_attributes()) {
            Ok(window) => window,
            Err(e) => {
                error!("error: {:?}", e);
                return;
            }
        };
        info!("ウィンドウを作成しました");

        self.window = Some(window);
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                info!(
                    "キーボードイベントが発生しました。キーの状態: {:?}。logical_key: {:?}",
                    event.state, event.logical_key
                );
                if let Key::Named(s) = &event.logical_key {
                    if s == &NamedKey::Space {
                        match event.state {
                            ElementState::Pressed => {
                                info!("スペースキーが押下されました");
                                let window_ref = match &self.window {
                                    Some(window) => window,
                                    None => {
                                        error!("ウィンドウがありません");
                                        return;
                                    }
                                };
                                let window_handle = match window_ref.window_handle() {
                                    Ok(window_handle) => window_handle,
                                    Err(e) => {
                                        error!("エラーが起きました: {:?}", e);
                                        return;
                                    }
                                };
                                let window_raw = window_handle.as_raw();
                                match window_raw {
                                    RawWindowHandle::Win32(handle) => Some(handle.hwnd),
                                    _ => {
                                        error!("サポートされていない形式のウィンドウハンドルです");
                                        None
                                    }
                                };
                            }
                            ElementState::Released => {
                                info!("スペースキーが離されました");
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("ログを初期化しました");
    // イベントループとウィンドウの作成
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(e) => {
            error!("error: {:?}", e);
            return;
        }
    };
    let mut app = App::default();
    info!("イベントループを開始します");
    let _ = event_loop.run_app(&mut app);
    info!("アプリケーションを終了します");
}
