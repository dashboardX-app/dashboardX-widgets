// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    App, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM},
        windef::HWND,
    },
    um::winuser::{
        EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible, SetWindowLongPtrA,
        SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM, SWP_NOREDRAW, SWP_NOSIZE, WS_EX_NOACTIVATE,
    },
};

fn main() {
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let refresh = CustomMenuItem::new("refresh".to_string(), "Refresh");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_item(refresh)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_window_event(|event: GlobalWindowEvent| match event.event() {
            tauri::WindowEvent::Focused(_focused) => unsafe { 
                let mut handle = event.window().hwnd().unwrap().0;
                set_desktop(handle as HWND); 
            },
            _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "settings" => {
                    eprintln!("Open settings")
                }
                "refresh" => {
                    let window = app.get_window("main").unwrap();
                    window.emit("refresh", {}).unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

unsafe fn set_desktop(window_handle: HWND) {
    eprintln!("Setting window position and attributes to desktop.");

    SetWindowPos(
        window_handle,
        HWND_BOTTOM,
        0,
        0,
        0,
        0,
        SWP_NOSIZE | 0x0010 | SWP_NOREDRAW,
    );

    SetWindowLongPtrA(window_handle, GWL_EXSTYLE, WS_EX_NOACTIVATE as isize);
}

/***** Old method to get HWND *****/

/*fn set_desktop() {
    // Call EnumWindows to enumerate open windows
    unsafe {
        EnumWindows(Some(enum_windows_callback), 0 as LPARAM);
    }
}

unsafe extern "system" fn enum_windows_callback(window_handle: HWND, _lparam: LPARAM) -> BOOL {
    let pid = std::process::id();

    // Check if the window is visible
    if unsafe { IsWindowVisible(window_handle) } == 0 {
        return 1; // Continue enumeration
    }

    // Get the window title
    let mut window_title = [0u16; 1024];
    unsafe {
        GetWindowTextW(
            window_handle,
            window_title.as_mut_ptr(),
            window_title.len() as i32,
        );
    }

    // Get the process ID associated with the window
    let mut process_id = 0;
    unsafe {
        GetWindowThreadProcessId(window_handle, &mut process_id);
    }
    let raw_title = String::from_utf16(&window_title)
        .ok()
        .unwrap_or_else(|| "".to_string());

    let (title, _garbage) = raw_title.split_once("\0").unwrap_or(("", ""));

    if process_id == pid && title == "dashboardX" {
        // Do something with the window handle (hwnd), window title, and process ID

        eprintln!(
            "HWND: {:?}, Process ID: {:?}, Window Title: {:?}",
            window_handle, process_id, title
        );

        set_pos_attr(window_handle);

    }
    1 // Continue enumeration
}*/
