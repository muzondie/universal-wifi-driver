use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::Controls::*;
use windows::core::*;
use log::{info, error};

pub struct App {
    hwnd: HWND,
}

impl App {
    pub fn new() -> Result<Self, windows::core::Error> {
        unsafe {
            let instance = HINSTANCE(GetModuleHandleA(None).unwrap().0);
            let class_name = s!("UniversalWifiDriver");
            
            let wc = WNDCLASSEXA {
                cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                hInstance: instance,
                lpszClassName: class_name,
                ..Default::default()
            };
            
            RegisterClassExA(&wc);
            
            let hwnd = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                class_name,
                s!("Universal WiFi Driver"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                400,
                300,
                None,
                None,
                instance,
                None,
            );
            
            Ok(Self { hwnd })
        }
    }

    pub fn run(&self) -> windows::core::Result<()> {
        unsafe {
            let mut msg = MSG::default();
            while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            Ok(())
        }
    }

    extern "system" fn wndproc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match msg {
                WM_CREATE => {
                    CreateWindowA(
                        s!("BUTTON"),
                        s!("Scan and Install"),
                        WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON,
                        10,
                        10,
                        120,
                        30,
                        hwnd,
                        HMENU(1),
                        HINSTANCE(0),
                        None,
                    );
                    
                    LRESULT(0)
                }
                WM_COMMAND => {
                    if HIWORD(wparam.0 as u32) == BN_CLICKED {
                        std::thread::spawn(|| {
                            let mut manager = driver_manager::DriverManager::new();
                            if let Err(e) = manager.install_drivers(vec![]) {
                                error!("Install failed: {}", e);
                            }
                        });
                    }
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(hwnd, msg, wparam, lparam),
            }
        }
    }

    pub fn init_logger() {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}] {}",
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    }
}