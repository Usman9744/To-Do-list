use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::io::{Read, BufReader, BufRead}; 

fn main() {
    println!("📋 Smart To-Do List");

    loop {
        println!("\nChoose an option:");
        println!("1️⃣ Add a task");
        println!("2️⃣ View tasks");
        println!("3️⃣ Remove a task");
        println!("4️⃣ Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => add_task(),
            "2" => view_tasks(),
            "3" => remove_task(),
            "4" => {
                println!("Goodbye! 👋");
                break;
            },
            _ => println!("❌ Invalid choice, try again!"),
        }
    }
}

// Function to add a task
fn add_task() {
    let mut task = String::new();
    
    println!("Enter the task you want to add:");
    io::stdin().read_line(&mut task).expect("Failed to read input");

    let task = task.trim();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Failed to open tasks.txt");

    writeln!(file, "{}", task).expect("Failed to write to tasks.txt");

    println!("✅ Task added: {}", task);
}

// Function to view tasks
fn view_tasks() {
    let file = File::open("tasks.txt");

    match file {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content).expect("Failed to read the file");
            if content.is_empty() {
                println!("❌ No tasks available.");
            } else {
                println!("📜 Tasks:");
                println!("{}", content);
            }
        }
        Err(_) => {
            println!("❌ No tasks file found.");
        }
    }
}

// Function to remove a task
fn remove_task() {
    let file = match File::open("tasks.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("❌ No tasks file found.");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut tasks: Vec<String> = Vec::new();
    for line in reader.lines() {
        tasks.push(line.unwrap());
    }

    if tasks.is_empty() {
        println!("❌ No tasks available.");
        return;
    }

    println!("📜 Tasks to choose from:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {}", index + 1, task);
    }

    let mut choice = String::new();
    println!("Enter the number of the task you want to remove:");
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: usize = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid number, try again.");
            return;
        }
    };

    if choice == 0 || choice > tasks.len() {
        println!("❌ Invalid task number.");
        return;
    }

    tasks.remove(choice - 1);  // Remove the selected task

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.txt")
        .expect("Failed to open tasks.txt");

    for task in tasks {
        writeln!(file, "{}", task).expect("Failed to write task to file");
    }

    println!("✅ Task removed!");
}
