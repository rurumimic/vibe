//! Task queue example using ring buffer.
//!
//! Demonstrates a simple task processing queue with bounded capacity.

use ring_buffer::{Error, Queue};

/// Represents a task to be processed.
#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
}

impl Task {
    fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    fn process(&self) {
        println!("Processing task #{}: {}", self.id, self.name);
    }
}

fn main() {
    let mut task_queue: Queue<Task> = Queue::new(3);

    // Enqueue tasks
    let tasks = [
        Task::new(1, "Send email"),
        Task::new(2, "Generate report"),
        Task::new(3, "Backup database"),
    ];

    for task in tasks {
        match task_queue.push(task) {
            Ok(()) => println!("Task enqueued"),
            Err(Error::Full) => println!("Queue full, task dropped"),
        }
    }

    println!("\nPending tasks: {}", task_queue.len());

    // Try to add one more task to a full queue
    let extra_task = Task::new(4, "Clean logs");
    if let Err(Error::Full) = task_queue.push(extra_task) {
        println!("Cannot add more tasks, queue is full\n");
    }

    // Process all tasks
    while let Some(task) = task_queue.pop() {
        task.process();
    }

    println!("\nAll tasks completed. Queue empty: {}", task_queue.is_empty());
}
