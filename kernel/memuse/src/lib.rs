#![no_std]
#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate terminal_print;
extern crate task;
extern crate heap;

use core::ops::Deref;
use alloc::string::String;

pub fn memuse(task_id: usize, mem_type: i8) -> Result<(), String> {
   // let _res = logger::write_fmt(format_args!("task_id: {}\nmem_type: {}\n", task_id, mem_type));
    info!("task_id: {}\n", task_id);
    info!("mem_type: {}\n", mem_type);
    let task_ref = task::get_task(task_id);
    match task_ref {
        Some(tr) => {
            let result = match mem_type {
                0 => {
                    heap::get_heap_size(task_id)
                }
                1 => {
                    tr.deref().get_stack_size()
                }
                _ => {
                    return Err(format!("Only mem_type 0 (heap) and 1 (stack) supported"));
                }
            };
            println!("Requested memory size is: {}\n", result);
            Ok(())
        }
        _ => {
            Err(format!("No task with id {} found", task_id))
        }
    }
}
