pub mod Tasker {

	pub enum TaskStatus {
		ICEBOX,
		INPROGRESS,
		DONE
	}
	pub struct Task {
		pub title: String,
		pub status: TaskStatus,
		id: u32
		// adding descriptors in the future would be nice, but I might just allow this to be extended.
	}
	pub struct TaskStack {
		tasks: Vec<Task>,
		id_counter: u32,
		// This is right now stateful which I don't think I want.
		//Maybe introduce a seperate function that mutates objects? not really sure right now.
	}

	impl TaskStack {
		pub fn new() -> Self {
			TaskStack {
				tasks: Vec::new(),
				id_counter: 0,
			}
		}


		pub fn push(&mut self, task: Task) -> Result<&Task, u8> {
			task.id = self.id_counter;
			self.id_counter += 1
			self.tasks.push(task);
			Ok(&task)
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


}

mod Timers {

	use crate::Tasker::{Task, TaskStack};
	use std::time::{SystemTime, SystemTimeError, Duration};
	struct WorkSession {
		start: SystemTime,
		tasks: TaskStack
	}

	impl WorkSession {
		pub fn start(start: u32, tasks: TaskStack) -> Self {
			WorkSession {
				start: SystemTime::now(),
				tasks,
			}
		}
		pub fn stop(&self) -> (TaskStack, Result<Duration, SystemTimeError>){
			//weird but it works
			(
				self.tasks, 
				SystemTime::duration_since(&self.start,SystemTime::now())
			)
		}

		pub fn add_task(&mut self, task: Task) {
			self.tasks.push(task);
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::Tasker::*;
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

		if let Ok(&Task) = task_queue.push(task) {
			assert_eq!(5, 5);
		} else {
			assert_eq!(4, 5);
		}
		// match task_queue.push(task) {
		// 	Ok()
		// }
	}

}
