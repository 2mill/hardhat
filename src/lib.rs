pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub enum TaskStatus {
	ICEBOX,
	INPROGRESS,
	DONE
}

pub struct Task {
	title: String,
	status: TaskStatus,
	id: u32
}

pub struct TaskStack {
	tasks: Vec<Task>
}

impl TaskStack {
	pub fn new() -> Self {
		TaskStack {
			tasks: Vec::new(),
		}
	}

	pub fn push(&mut self, task: Task) {
		self.tasks.push(task)
	}
}


impl Task {
	pub fn new(title: String, status: TaskStatus) -> Self {
		Task {
			title,
			status,
			id: 0
		}
	}

	pub fn set_id(&mut self, id: u32) {
		self.id = id
	}
}

mod Timers {
	use std::os::unix;

	use crate::TaskStack;
	use crate::Task;
	struct Timer {
		start: u32,
		stop: u32,
		tasks: TaskStack
	}

	impl Timer {
		pub fn start(start: u32) -> Self {
			Timer {
				start,
				stop: 0,
				tasks: TaskStack::new()
			}
		}
		pub fn stop(&mut self) {
			self.stop = 0;
		}

		pub fn add_task(&mut self, task: Task) {
			self.tasks.push(task);
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

	#[test]
	fn build_task() {
		let task = Task::new(
			String::from("Fizz Buzz"),
			TaskStatus::ICEBOX
		);

		assert_eq!(task.title, String::from("Fizz Buzz"))

	}

	#[test]
	fn build_task_queue_and_task() {
		let task = Task::new(
			String::from("Fizz Buzz"),
			TaskStatus::DONE
		);

		let mut task_queue = TaskStack::new();
		task_queue.push(task)
	}

}
