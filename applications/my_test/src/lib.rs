#![no_std]

extern crate memuse;
#[macro_use] extern crate terminal_print;
#[macro_use] extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

pub fn main(args: Vec<String>) -> isize {
    assert_eq!(args.len(), 2);
    let task_id: usize = args[0].parse().unwrap();
    let mem_type: i8 = args[1].parse().unwrap();
    let result = memuse::memuse(task_id, mem_type);
    match result {
        Ok(_) => {
            println!("{}", format!("Task {} found", task_id));
        }
        Err(_msg) => {
            println!("{}", _msg);
        }
    }
    0
}
