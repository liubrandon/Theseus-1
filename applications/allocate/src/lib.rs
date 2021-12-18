#![no_std]

extern crate app_io;
extern crate task;
#[macro_use] extern crate terminal_print;
#[macro_use] extern crate alloc;

use alloc::string::String;

/// Helper function to get the PID of current task
fn getpid() -> usize {
	let taskref = match task::get_my_current_task() {
        Some(t) => t,
        None => {
            println!("failed to get current task");
            return 0;
        }
    };

    taskref.id
}

fn recurse(allocated: u128, depth: u128) -> Result<(), &'static str> {
    println!("{}", format!("Allocated {} bytes\nRecursion depth {}\nPid: {}", allocated, depth, getpid()));
    let mut stdin = app_io::stdin()?;
    let mut arr: [char; 4097] = ['a'; 4097];
    arr[10] = 'b';
    let _str = String::from("aslkdjalskdjalskjdlaskjdlakjsdlaksjdlaksjdlaksjdlaskjdalskdjalskjdalskdjalskdjalskdjalskdjalskdjalskdjlaskdjalskjdalksjdlakjsdlakjs");
    let mut buf = String::new();
    let _cnt = stdin.read_line(&mut buf).or(Err("failed to perform read_line"))?;
    buf.clear();
    if let Err(_e) = recurse(allocated, depth+1) {
        return Ok(());
    }
    Ok(())
}

pub fn main() -> isize {
    if let Err(_e) = recurse(0, 0) {
        return 1;
    }
    0
}
