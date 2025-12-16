// Include the file whose name contains hyphens by specifying its path
#[path = "without-serde-derive.rs"]
mod without_serde_derive;

use without_serde_derive::test;

fn main() {
    // clear the terminal (attempt platform-specific command, fallback to ANSI)
    clear_screen();

    // call the public function from the module
    test();
}

fn clear_screen() {
    // On Windows, use `cmd /C cls`. On Unix-like systems, try `clear`.
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status();
    } else {
        // Try external `clear` command first; if it fails, print ANSI escape
        match std::process::Command::new("clear").status() {
            Ok(_) => {
                println!("Cleared screen using 'clear' command.");
                print!("\x1B[2J\x1B[1;1H");
            }
            Err(_) => {
                // Fallback: ANSI clear screen + move cursor to 1;1
                print!("\x1B[2J\x1B[1;1H");
            }
        }
    }
}