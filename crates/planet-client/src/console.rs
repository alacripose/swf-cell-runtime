//! The debug console: the window that must be open while the game window is.
//!
//! The game window cannot be opened without it. [`DebugConsole::require`] holds a console before a
//! game window exists and answers [`ConsoleRefusal`] (`debug-console-absent`) when it cannot: the
//! caller propagates that refusal, so a run with no visible debug console fails instead of starting
//! a game whose output goes nowhere. That is the rule, not a preference — `tools/port_gate.py`'s
//! `debug_console` check refuses a game entry point that opens a window first.
//!
//! On Windows the console is the `cmd` window: an inherited one is held, and when the process has
//! none `AllocConsole` creates one. Either way `GetConsoleWindow` is asked again after acquisition,
//! so "a console exists" is observed rather than assumed, the title is set so the window is
//! identifiable apart from the game, and the process's output handles are pointed at the console so
//! the debug lines appear **in that window** even when the process was started through pipes.
//!
//! On other platforms the console is the terminal the process was started from: a terminal on
//! stderr is held; a run whose debug output is redirected is refused, because this client cannot
//! create a console window there and reporting one it does not have would be the lie this exists to
//! prevent.

/// The console could not be held, so the game window must not be opened.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsoleRefusal {
    pub code: &'static str,
    pub message: String,
}

impl std::fmt::Display for ConsoleRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ConsoleRefusal {}

/// A held debug console, kept alive for as long as the game window is.
///
/// Holding the value is the point: it is created before the window and lives until the process
/// ends, so the console is never dropped out from under the game.
#[derive(Debug)]
pub struct DebugConsole {
    title: &'static str,
    origin: &'static str,
}

impl DebugConsole {
    /// **Hold the debug console, or refuse.** The caller propagates the refusal, which happens
    /// before any game window is created.
    pub fn require() -> Result<DebugConsole, ConsoleRefusal> {
        match platform::acquire() {
            Ok((title, origin)) => {
                eprintln!("debug console held ({origin}): {title}");
                eprintln!(
                    "this window is part of the client, not an option: the game window is \
                     refused without it"
                );
                Ok(DebugConsole { title, origin })
            }
            Err(reason) => Err(ConsoleRefusal {
                code: "debug-console-absent",
                message: format!(
                    "{reason}; the game window is not opened without the debug console"
                ),
            }),
        }
    }

    /// The title the console was given, so a caller can state what it is showing in.
    pub fn title(&self) -> &'static str {
        self.title
    }

    /// How the console was obtained: inherited, or created by the client.
    pub fn origin(&self) -> &'static str {
        self.origin
    }
}

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;
    use std::ptr;

    /// The console window's title, so the debug window is not mistaken for the game itself.
    pub const TITLE: &str = "planet client — debug console";

    const GENERIC_READ: u32 = 0x8000_0000;
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const FILE_SHARE_READ_WRITE: u32 = 0x1 | 0x2;
    const OPEN_EXISTING: u32 = 3;
    const STD_OUTPUT_HANDLE: u32 = 0xFFFF_FFF5; // -11
    const STD_ERROR_HANDLE: u32 = 0xFFFF_FFF4; // -12
    const INVALID_HANDLE_VALUE: *mut c_void = -1isize as *mut c_void;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetConsoleWindow() -> *mut c_void;
        fn AllocConsole() -> i32;
        fn SetConsoleTitleW(title: *const u16) -> i32;
        fn CreateFileW(
            name: *const u16,
            access: u32,
            share: u32,
            security: *mut c_void,
            creation: u32,
            flags: u32,
            template: *mut c_void,
        ) -> *mut c_void;
        fn SetStdHandle(which: u32, handle: *mut c_void) -> i32;
    }

    /// Point one standard handle at the console, so `eprintln!` lands in the console window even
    /// when the process was started with its output redirected elsewhere.
    ///
    /// The handle is deliberately not closed: it is the process's output handle for the rest of the
    /// run, and the operating system closes it at exit.
    fn point_at_console(which: u32) {
        let mut name: Vec<u16> = "CONOUT$".encode_utf16().collect();
        name.push(0);
        // SAFETY: `name` is a NUL-terminated wide string that outlives the call, and the remaining
        // arguments are the documented constant values for opening a console output handle.
        unsafe {
            let handle = CreateFileW(
                name.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                0,
                ptr::null_mut(),
            );
            if handle != INVALID_HANDLE_VALUE && !handle.is_null() {
                SetStdHandle(which, handle);
            }
        }
    }

    /// Hold the inherited `cmd` window, or create one, then **check that it exists**.
    pub fn acquire() -> Result<(&'static str, &'static str), String> {
        // SAFETY: all of these are the documented kernel32 console entry points, called with the
        // arguments their signatures state; the title buffer outlives the call.
        let inherited = unsafe { !GetConsoleWindow().is_null() };
        unsafe {
            if !inherited && AllocConsole() == 0 {
                return Err(
                    "AllocConsole failed and the process inherited no console window".to_string(),
                );
            }
            if GetConsoleWindow().is_null() {
                return Err("no console window exists after acquisition".to_string());
            }
            let mut title: Vec<u16> = TITLE.encode_utf16().collect();
            title.push(0);
            SetConsoleTitleW(title.as_ptr());
        }
        point_at_console(STD_OUTPUT_HANDLE);
        point_at_console(STD_ERROR_HANDLE);
        Ok((
            TITLE,
            if inherited {
                "inherited console window"
            } else {
                "opened a console window with AllocConsole"
            },
        ))
    }
}

#[cfg(not(windows))]
mod platform {
    use std::io::IsTerminal;

    /// The console is the terminal the process was started from.
    pub const TITLE: &str = "planet client — debug console";

    /// Hold the terminal, or refuse: this platform cannot create a console window.
    pub fn acquire() -> Result<(&'static str, &'static str), String> {
        if std::io::stderr().is_terminal() {
            Ok((TITLE, "inherited terminal on stderr"))
        } else {
            Err(
                "stderr is not a terminal and this platform cannot open a console window"
                    .to_string(),
            )
        }
    }
}
