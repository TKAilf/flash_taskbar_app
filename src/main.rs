use std::fs::File;
use std::io::Read;
use std::path::Path;

use env_logger::Env;
use log::{error, info};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FlashWindowEx, FLASHWINFO, FLASHW_TRAY};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Icon, Window, WindowId};

/// アプリケーションの状態を保持する構造体です。
#[derive(Default)]
struct App {
    window: Option<Window>,
}

/// App構造体に対するApplicationHandlerトレイトの実装。
impl ApplicationHandler for App {
    /// アプリケーションが再開されたときに呼び出されるメソッドです。
    /// 新しいウィンドウを作成し、それを `window` フィールドに割り当てます。
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(Window::default_attributes()) {
            Ok(window) => window,
            Err(e) => {
                error!("error: {:?}", e);
                return;
            }
        };
        info!("ウィンドウを作成しました");

        // アイコンを読み込む
        let icon_bytes = load_icon_from_file("assets/icon.ico");
        let app_icon = load_icon(&icon_bytes);
        window.set_window_icon(Some(app_icon));
        info!("アイコンを設定しました");

        self.window = Some(window);
    }

    /// キーボード入力などのウィンドウイベントを処理します。
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
                                let winit_window_ref = match &self.window {
                                    Some(window) => window,
                                    None => {
                                        error!("ウィンドウがありません");
                                        return;
                                    }
                                };
                                let winit_window_handle = match winit_window_ref.window_handle() {
                                    Ok(window_handle) => window_handle,
                                    Err(e) => {
                                        error!("エラーが起きました: {:?}", e);
                                        return;
                                    }
                                };
                                let winit_window_raw = winit_window_handle.as_raw();
                                let winit_window_hwnd = match winit_window_raw {
                                    RawWindowHandle::Win32(handle) => {
                                        HWND(handle.hwnd.get() as isize)
                                    }
                                    _ => {
                                        error!("Windows 以外のプラットフォームです");
                                        return;
                                    }
                                };
                                flash_window(winit_window_hwnd);
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

/// タスクバーのウィンドウを点滅させてユーザーの注意を引く関数です。
///
/// # 引数
///
/// * `hwnd` - 点滅させるウィンドウのハンドル
fn flash_window(hwnd: HWND) {
    unsafe {
        let mut flash_info = FLASHWINFO {
            cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
            hwnd,
            dwFlags: FLASHW_TRAY,
            uCount: 5,    // 点滅回数
            dwTimeout: 0, // デフォルトのタイムアウト
        };

        let result = FlashWindowEx(&mut flash_info);
        if result.as_bool() {
            println!("ウィンドウの点滅に成功しました");
        } else {
            eprintln!("ウィンドウの点滅に失敗しました");
        }
    }
}

/// ファイルからアイコンをバイト配列で読み込みます。
///
/// # 引数
///
/// * `path` - 開きたいアイコンファイルのパスを表す文字列スライス。
///
/// # パニック
///
/// ファイルのオープンに失敗した場合は、"Failed to open icon file" というメッセージとともにパニックを引き起こします。
fn load_icon_from_file(path: &str) -> Vec<u8> {
    let mut file = File::open(Path::new(path)).expect("Failed to open icon file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read icon file");
    buffer
}

/// メモリから画像を読み込み、Iconを取得します。
///
/// # パラメータ
/// - `bytes`: 画像データのバイト配列。
///
/// # 戻り値
/// - `Icon`: RGBAデータ、幅、高さを保持する`Icon`構造体。
///
/// # パニック
/// 画像の読み込みに失敗した場合、`"Failed to load image from memory"`というメッセージと共にパニックします。
/// アイコンのオープンに失敗した場合、`"Failed to open icon"`というメッセージと共にパニックします。
fn load_icon(bytes: &[u8]) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(bytes)
            .expect("Failed to load image from memory")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

/// エントリーポイントです。
/// ロガーを初期化し、イベントループを作成し、アプリケーションを実行します。
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
    let result = event_loop.run_app(&mut app);
    if let Err(e) = result {
        error!("error: {:?}", e);
    }
    info!("アプリケーションを終了します");
}
