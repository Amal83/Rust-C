use std::io::{self, Write, BufRead, BufReader};
use std::fs::{File, OpenOptions};

struct Task {
    description: String,
    done: bool,
}

fn main() {
    let mut tasks = load_tasks();
    println!("Welcome to Rust To-Do CLI!");

    loop {
        println!("\nMenu:");
        println!("1. Add task");
        println!("2. Mark task as done");
        println!("3. List tasks");
        println!("4. Quit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                add_task(&mut tasks);
                save_tasks(&tasks);
            },
            "2" => {
                mark_done(&mut tasks);
                save_tasks(&tasks);
            },
            "3" => list_tasks(&tasks),
            "4" => break,
            _ => println!("Invalid option!"),
        }
    }

}

fn save_tasks(tasks: &Vec<Task>) {
    let mut file = File::create("tasks.txt").unwrap();

    for task in tasks {
        let line = format!("{}|{}\n", task.done, task.description);
        file.write_all(line.as_bytes()).unwrap();
    }
}

fn load_tasks() -> Vec<Task> {
    let file = OpenOptions::new().read(true).open("tasks.txt");

    let mut tasks = Vec::new();

    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    let done = parts[0] == "true";
                    let description = parts[1].to_string();

                    tasks.push(Task { description, done });
                }
            }
        }
    }

    tasks
}

fn add_task(tasks: &mut Vec<Task>) {
    print!("Enter a new task: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if !input.is_empty() {
        tasks.push(Task {
            description: input.to_string(),
            done: false,
        });
    }
}

fn mark_done(tasks: &mut Vec<Task>) {
    list_tasks(tasks);
    print!("Enter task number to mark done: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(num) = input.trim().parse::<usize>() {
        if num > 0 && num <= tasks.len() {
            tasks[num - 1].done = true;
        } else {
            println!("Invalid task number");
        }
    } else {
        println!("Please enter a number");
    }
}

fn list_tasks(tasks: &Vec<Task>) {
    println!("\nCurrent Tasks:");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: [{}] {}", i + 1, if task.done { "x" } else { " " }, task.description);
    }
}