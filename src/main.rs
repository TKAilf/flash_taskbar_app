use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{FLASHWINFO, FLASHW_ALL, FlashWindowEx},
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
};

fn main() {
    // イベントループとウィンドウの作成
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("FlashWindowEx Example")
        .build(&event_loop)
        .unwrap();

    // ウィンドウハンドルの取得
    let hwnd = HWND(window.hwnd() as isize);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                // ウィンドウが閉じられた場合の処理
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                // キーボードのスペースキーが押された場合の処理
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(virtual_keycode) = input.virtual_keycode {
                        if virtual_keycode == winit::event::VirtualKeyCode::Space {
                            // FlashWindowExの呼び出し
                            let mut flash_info = FLASHWINFO {
                                cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
                                hwnd,
                                dwFlags: FLASHW_ALL,
                                uCount: 3,       // 点滅回数
                                dwTimeout: 0,    // デフォルトの点滅間隔
                            };
                            unsafe {
                                FlashWindowEx(&mut flash_info);
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
}
