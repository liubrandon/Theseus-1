#![no_std]
#[macro_use] extern crate alloc;
#[macro_use] extern crate log;

extern crate task;

use alloc::string::String;

pub fn memuse(task_id: usize, mem_type: i8) -> Result<(), String> {
   // let _res = logger::write_fmt(format_args!("task_id: {}\nmem_type: {}\n", task_id, mem_type));
    info!("task_id: {}\n", task_id);
    info!("mem_type: {}\n", mem_type);
    let task_ref = task::get_task(task_id);
    match task_ref {
        Some(_x) => {
            Ok(())
        }
        _ => {
            Err(format!("No task with id {} found", task_id))
        }
    }
}
