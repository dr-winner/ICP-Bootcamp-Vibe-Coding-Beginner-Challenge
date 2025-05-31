// Starter code for the Rust Task Manager challenge

use std::fs::{self, File};
use std::io::{self, Write, BufReader, BufRead};
use std::collections::HashMap;
use std::error::Error;

// Task status enum
#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
}

// Task struct to store task information
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
}

impl Task {
    fn new(id: u32, title: String, description: String, due_date: Option<String>) -> Task {
        Task {
            id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        }
    }

    fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }

    fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Completed)
    }

    fn display(&self) {
        println!("\nTask #{}", self.id);
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Status: {:?}", self.status);
        if let Some(date) = &self.due_date {
            println!("Due Date: {}", date);
        }
    }
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
        let task = Task::new(self.next_id, title, description, due_date);
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }
        for task in &self.tasks {
            task.display();
        }
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.complete();
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() == initial_len {
            Err(format!("Task with ID {} not found", id))
        } else {
            Ok(())
        }
    }

    fn filter_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.status == status)
            .collect()
    }

    fn filter_by_due_date(&self, date: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.due_date.as_ref().map_or(false, |d| d == date))
            .collect()
    }

    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        for task in &self.tasks {
            writeln!(file, "{}|{}|{}|{}|{:?}",
                task.id,
                task.title,
                task.description,
                task.due_date.as_deref().unwrap_or(""),
                task.status
            )?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        
        self.tasks.clear();
        self.next_id = 1;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            
            if parts.len() >= 5 {
                let id = parts[0].parse::<u32>()?;
                let title = parts[1].to_string();
                let description = parts[2].to_string();
                let due_date = if parts[3].is_empty() {
                    None
                } else {
                    Some(parts[3].to_string())
                };
                let status = if parts[4] == "Completed" {
                    TaskStatus::Completed
                } else {
                    TaskStatus::Pending
                };

                let task = Task {
                    id,
                    title,
                    description,
                    due_date,
                    status,
                };
                
                self.tasks.push(task);
                self.next_id = self.next_id.max(id + 1);
            }
        }
        Ok(())
    }
}

// Command enum to represent user commands
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Quit,
    Unknown,
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    match parts.get(0).map(|&s| s.to_lowercase()) {
        Some(cmd) => match cmd.as_str() {
            "add" => {
                if parts.len() < 3 {
                    return Command::Unknown;
                }
                let title = parts[1].to_string();
                let description = parts[2].to_string();
                let due_date = parts.get(3).map(|&s| s.to_string());
                Command::Add { title, description, due_date }
            }
            "list" => Command::List,
            "complete" => {
                if let Some(id) = parts.get(1).and_then(|s| s.parse::<u32>().ok()) {
                    Command::Complete { id }
                } else {
                    Command::Unknown
                }
            }
            "delete" => {
                if let Some(id) = parts.get(1).and_then(|s| s.parse::<u32>().ok()) {
                    Command::Delete { id }
                } else {
                    Command::Unknown
                }
            }
            "save" => {
                if let Some(filename) = parts.get(1) {
                    Command::Save { filename: filename.to_string() }
                } else {
                    Command::Unknown
                }
            }
            "load" => {
                if let Some(filename) = parts.get(1) {
                    Command::Load { filename: filename.to_string() }
                } else {
                    Command::Unknown
                }
            }
            "quit" => Command::Quit,
            _ => Command::Unknown,
        },
        None => Command::Unknown,
    }
}

fn main() {
    let mut task_manager = TaskManager::new();
    
    println!("Welcome to the Task Manager!");
    println!("Available commands:");
    println!("  add <title> <description> [due_date]");
    println!("  list");
    println!("  complete <id>");
    println!("  delete <id>");
    println!("  save <filename>");
    println!("  load <filename>");
    println!("  quit");
    
    loop {
        let input = get_user_input("\nEnter command: ");
        let command = parse_command(&input);
        
        match command {
            Command::Add { title, description, due_date } => {
                task_manager.add_task(title, description, due_date);
                println!("Task added successfully!");
            }
            Command::List => {
                task_manager.list_tasks();
            }
            Command::Complete { id } => {
                match task_manager.complete_task(id) {
                    Ok(_) => println!("Task marked as completed!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Delete { id } => {
                match task_manager.delete_task(id) {
                    Ok(_) => println!("Task deleted successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Save { filename } => {
                match task_manager.save_to_file(&filename) {
                    Ok(_) => println!("Tasks saved successfully!"),
                    Err(e) => println!("Error saving tasks: {}", e),
                }
            }
            Command::Load { filename } => {
                match task_manager.load_from_file(&filename) {
                    Ok(_) => println!("Tasks loaded successfully!"),
                    Err(e) => println!("Error loading tasks: {}", e),
                }
            }
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
            Command::Unknown => {
                println!("Unknown command. Please try again.");
            }
        }
    }
}