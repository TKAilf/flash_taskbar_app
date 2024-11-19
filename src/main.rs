use log::{error, info};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::Key;
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

        self.window = Some(window);
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let Key::Character(s) = &event.logical_key {
                    if s == " " {
                        match event.state {
                            ElementState::Pressed => {
                                info!("スペースキーが押下されました");
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
    env_logger::init();
    // イベントループとウィンドウの作成
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
