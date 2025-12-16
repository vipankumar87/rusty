// Include the file whose name contains hyphens by specifying its path
#[path = "without-serde-derive.rs"]
mod without_serde_derive;

#[path="my-utils.rs"]
mod my_utils;

use without_serde_derive::test;
use my_utils::clear_screen;

fn main() {
    // clear the terminal (attempt platform-specific command, fallback to ANSI)
    clear_screen();

    // call the public function from the module
    test();
    std::thread::sleep(std::time::Duration::from_secs(1));
    clear_screen();

    let countdown = [1,2,4,5,6];
    let number = countdown[2];
    let number  = number + 2;
    println!("Final number: {}", number);

    let number = countdown.get(3); /// this thing return Option<&i32> => is enum not value
    // let number = match number {
    //     Some(&num) => num + 3,
    //     None => 0,
    // };
    // let number = number.map(|&num| num + 3).unwrap_or(0);
    // let number = number.unwrap_or(&0) + 3;
    // let number = number.copied().unwrap_or(0) + 3;
    // let number = number.cloned().unwrap_or(0) + 3;
    let number = number.unwrap() + 3;
    println!("Final number after get: {}", number);
}