pub fn clear_screen() {
    // On Windows, use `cmd /C cls`. On Unix-like systems, try `clear`.
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status();
    } else {
        // Try external `clear` command first; if it fails, print ANSI escape
        match std::process::Command::new("clear").status() {
            Ok(_) => {
                print!("\x1B[2J\x1B[1;1H");
                println!("Cleared screen using 'clear' command.");
            }
            Err(_) => {
                // Fallback: ANSI clear screen + move cursor to 1;1
                print!("\x1B[2J\x1B[1;1H");
            }
        }
    }
}