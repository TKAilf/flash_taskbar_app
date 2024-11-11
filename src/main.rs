use eframe::egui::*;
use egui_winit_platform::{Platform, PlatformDescriptor};
use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{FlashWindowEx, FLASHWINFO, FLASHW_ALL, FLASHW_TIMERNOFG},
};
use winit::platform::windows::WindowExtWindows;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();

    // イベントループとウィンドウの作成
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("egui with winit example")
        .build(&event_loop)
        .unwrap();

    // ウィンドウハンドル（HWND）の取得
    let hwnd = window.hwnd() as HWND;

    // eguiのプラットフォームを初期化
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: window.inner_size().width,
        physical_height: window.inner_size().height,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Style::default(),
    });

    // Rendererの初期化（今回はeguiの描画は行いません）
    // 実際のアプリケーションでは、適切なレンダラーを使用してください

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // イベントをプラットフォームに転送
        platform.handle_event(&event);

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        // ウィンドウサイズ変更時の処理
                        platform.update_window_size(size.width, size.height);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                // eguiのフレームを開始
                platform.update_time(0.0); // 時間の更新が必要な場合は適切に設定

                platform.begin_frame();

                // UIの構築
                let ctx = platform.context();
                CentralPanel::default().show(&ctx, |ui| {
                    if ui.button("Flash Window").clicked() {
                        flash_window(hwnd);
                    }
                });

                let full_output = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(full_output.shapes);

                // ここで描画を行う（実際にはレンダラーが必要）
                // 例えば、egui_wgpu_backend などを使用して描画します

                // 次のフレームをリクエスト
                window.request_redraw();
            }
            _ => {}
        }
    });
}

// FlashWindowExを呼び出す関数
fn flash_window(hwnd: HWND) {
    unsafe {
        let mut flash_info = FLASHWINFO::default();
        flash_info.cbSize = std::mem::size_of::<FLASHWINFO>() as u32;
        flash_info.hwnd = hwnd;
        flash_info.dwFlags = FLASHW_ALL | FLASHW_TIMERNOFG;
        flash_info.uCount = 0; // 無限に点滅
        flash_info.dwTimeout = 0;

        FlashWindowEx(&flash_info);
    }
}
