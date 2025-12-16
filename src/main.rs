// Include the file whose name contains hyphens by specifying its path
#[path = "without-serde-derive.rs"]
mod without_serde_derive;

#[path="my-utils.rs"]
mod my_utils;

use without_serde_derive::test;
use my_utils::clear_screen;
use std::fs;
use std::io;
use chrono::Local;

struct User {
    id: i32,
}

/*

just for reference

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/

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

    let number = countdown.get(2); /// this thing return Option<&i32> => is enum not value
    let number = match number {
        Some(&num) => num + 3,
        None => 120,
    };
    // let number = number.map(|&num| num + 3).unwrap_or(0);
    // let number = number.unwrap_or(&100) +1; // 100 is fallback value if None
    // let number = number.copied().unwrap_or(0) + 3;
    // let number = number.cloned().unwrap_or(0) + 3;
    // let number = number.unwrap(&0) + 3; // this will not work
    // let number = number.unwrap() + 3; // this will work only if we are getting value within the index => countdown.get
    
    println!("Final number after get: {}", number);
    let mut num = 2;
    if let Some(&num) = countdown.get(2) {
        println!("Number from if let: {}", num);
    } else {
        println!("Index out of bounds");
    }

    // let number = Some(13);
    // if let Some(13) = number{
    //     println!("Thirteen!");
    // }

    // let number = Some(&13);

    // ❌ Hard / ugly / sometimes impossible


    // if Some(&13) == number {
    //     println!("Thirteen! 2");
    // }

    // let number = Some(User { id: 1 });

    // // ❌ Compile error: User doesn't implement PartialEq
    // if Some(User { id: 1 }) == number {
    //     println!("Matched User");
    // } else {
    //     println!("Not matched");

    // }

    let contents = fs::read_to_string("Cargo.toml").expect("Failed to read file");
    println!("File contents:\n{}", contents);

    // let contents = fs::read_to_string("non_existent_file.txt").expect("Failed to read file 111111111111111");
    // println!("File contents:\n{:?}", contents);


    let result = fs::read_to_string("non_existent_file.txt");
    let contents = match result {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file: {}", e);
            String::new() // or handle the error as needed
        }
    };
    println!("File contents:\n{:?}", contents);

    let result = fs::read_to_string("non_existent_file.txt");
    let contents = match result {
        Ok(data) => data,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("File not found.");
                String::from("File not found") // or handle the error as needed
            },
            io::ErrorKind::PermissionDenied => {
                println!("Permission denied.");
                String::from("Permission denied") // or handle the error as needed
            },
            _ => {
                println!("An unexpected error occurred: {}", e);
                String::new() // or handle the error as needed
            }
        }
    };
    println!("File contents:\n{:?}", contents);

    // let contents = fs::read_to_string("non_existent_file.txt").expect("Failed to read file 111111111111111");
    // println!("File contents:\n{:?}", contents);

    let current_date = Local::now();
    let formatted_date = current_date.format("%d-%m-%Y").to_string();
    let message = format!("testing {}", formatted_date);

    std::process::Command::new("git").args(["add","."]).status().unwrap();
    std::process::Command::new("git").args(["commit","-am", &message]).status().unwrap();
    std::process::Command::new("git").args(["push","--all"]).status().unwrap();
}