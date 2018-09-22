#![no_std]

extern crate libm;

mod memory;
mod monitor;

extern {
	pub fn swap_tasks();
	pub fn doneVas();
}

pub const CURRENT_TASKS: Vec<u64> = 0;
pub const CPU_USED: i16 = 0;

pub struct Task {
	deadline: i64,
	name: &str,
	est_cpu%: i16,
}

pub fn switch_task(prev_task: i16, new_task: i16) {
	if est_cpu_new + CPU_USED > 100 {
		
	} else if est_cpu_new + CPU_USED < 100 {
		get_specs!();
		
		let task: Task = Task {
			deadline,
			name,
			est_cpu_new,
		}
		
		unsafe {
			CURRENT_TASK.drop(prev_task);
			CURRENT_TASK.push(new_task);
			
			CPU_USED += CPU_USED + est_cpu_new;
		}
	}
}
