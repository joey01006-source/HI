// Windows: install Rust, then run `rustc square.rs` and double-click square.exe.
// Opens a standalone window containing a blue square; close the window to exit.
#![windows_subsystem = "windows"]
#![cfg_attr(not(target_os = "windows"), allow(dead_code))]

#[cfg(not(target_os = "windows"))]
compile_error!("This program requires Windows. Build with rustc square.rs on Windows.");

#[cfg(target_os = "windows")]
mod game {
    use std::ffi::c_void;
    use std::ptr::{null, null_mut};

    type Handle = *mut c_void;

    #[repr(C)]
    struct Point { x: i32, y: i32 }
    #[repr(C)]
    struct Rect { left: i32, top: i32, right: i32, bottom: i32 }
    #[repr(C)]
    struct Paint {
        dc: Handle,
        erase: i32,
        rect: Rect,
        restore: i32,
        update: i32,
        reserved: [u8; 32],
    }
    #[repr(C)]
    struct Message {
        window: Handle,
        message: u32,
        wparam: usize,
        lparam: isize,
        time: u32,
        point: Point,
        private: u32,
    }
    #[repr(C)]
    struct WindowClass {
        style: u32,
        procedure: unsafe extern "system" fn(Handle, u32, usize, isize) -> isize,
        class_extra: i32,
        window_extra: i32,
        instance: Handle,
        icon: Handle,
        cursor: Handle,
        background: Handle,
        menu_name: *const u16,
        class_name: *const u16,
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn RegisterClassW(class: *const WindowClass) -> u16;
        fn CreateWindowExW(ex: u32, class: *const u16, title: *const u16,
            style: u32, x: i32, y: i32, width: i32, height: i32,
            parent: Handle, menu: Handle, instance: Handle, param: Handle) -> Handle;
        fn DefWindowProcW(window: Handle, message: u32, w: usize, l: isize) -> isize;
        fn ShowWindow(window: Handle, command: i32) -> i32;
        fn GetMessageW(message: *mut Message, window: Handle, min: u32, max: u32) -> i32;
        fn TranslateMessage(message: *const Message) -> i32;
        fn DispatchMessageW(message: *const Message) -> isize;
        fn PostQuitMessage(code: i32);
        fn BeginPaint(window: Handle, paint: *mut Paint) -> Handle;
        fn EndPaint(window: Handle, paint: *const Paint) -> i32;
        fn GetClientRect(window: Handle, rect: *mut Rect) -> i32;
        fn FillRect(dc: Handle, rect: *const Rect, brush: Handle) -> i32;
        fn LoadCursorW(instance: Handle, name: *const u16) -> Handle;
        fn MessageBoxW(window: Handle, text: *const u16, title: *const u16, flags: u32) -> i32;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" { fn GetModuleHandleW(name: *const u16) -> Handle; }
    #[link(name = "gdi32")]
    unsafe extern "system" {
        fn CreateSolidBrush(color: u32) -> Handle;
        fn DeleteObject(object: Handle) -> i32;
    }

    fn wide(text: &str) -> Vec<u16> { text.encode_utf16().chain(Some(0)).collect() }

    // Windows owns the callback lifetime. All pointers passed here come from Win32.
    unsafe extern "system" fn procedure(window: Handle, message: u32, w: usize, l: isize) -> isize {
        unsafe {
            match message {
                0x000f => { // WM_PAINT
                    let mut paint: Paint = std::mem::zeroed();
                    let dc = BeginPaint(window, &mut paint);
                    let mut bounds: Rect = std::mem::zeroed();
                    GetClientRect(window, &mut bounds);
                    let x = (bounds.right - 200) / 2;
                    let y = (bounds.bottom - 200) / 2;
                    let square = Rect { left: x, top: y, right: x + 200, bottom: y + 200 };
                    let brush = CreateSolidBrush(40 | (120 << 8) | (220 << 16));
                    FillRect(dc, &square, brush);
                    DeleteObject(brush);
                    EndPaint(window, &paint);
                    0
                }
                0x0002 => { PostQuitMessage(0); 0 } // WM_DESTROY
                _ => DefWindowProcW(window, message, w, l),
            }
        }
    }

    pub fn run() -> Result<(), &'static str> {
        // Class/title buffers stay alive throughout the window's message loop.
        unsafe {
            let instance = GetModuleHandleW(null());
            let class_name = wide("RustSquareWindow");
            let title = wide("Rust Square");
            let class = WindowClass {
                style: 3, // Redraw after either dimension changes.
                procedure,
                class_extra: 0, window_extra: 0, instance,
                icon: null_mut(),
                cursor: LoadCursorW(null_mut(), 32512_usize as *const u16),
                background: 6_usize as Handle, // COLOR_WINDOW + 1
                menu_name: null(), class_name: class_name.as_ptr(),
            };
            if RegisterClassW(&class) == 0 { return Err("Could not register the window."); }
            let window = CreateWindowExW(0, class_name.as_ptr(), title.as_ptr(),
                0x00cf0000, i32::MIN, i32::MIN, 500, 500,
                null_mut(), null_mut(), instance, null_mut());
            if window.is_null() { return Err("Could not create the window."); }
            ShowWindow(window, 5);
            let mut message: Message = std::mem::zeroed();
            loop {
                match GetMessageW(&mut message, null_mut(), 0, 0) {
                    -1 => return Err("The window message loop failed."),
                    0 => return Ok(()),
                    _ => { TranslateMessage(&message); DispatchMessageW(&message); }
                }
            }
        }
    }

    pub fn show_error(text: &str) {
        unsafe { MessageBoxW(null_mut(), wide(text).as_ptr(), wide("Rust Square").as_ptr(), 0x10); }
    }
}

#[cfg(target_os = "windows")]
fn main() {
    if let Err(error) = game::run() {
        game::show_error(error);
        std::process::exit(1);
    }
}
