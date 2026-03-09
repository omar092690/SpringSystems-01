use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
} 

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let result = Command::new("ls").arg(path).status();

            match result {
                Ok(status) => {
                    if !status.success() {
                        println!("Failed to list files.");
                    }
                }
                Err(_) => println!("Failed to execute ls."),
            }
        }

        FileOperation::Display(path) => {
            let result = Command::new("cat").arg(path).status();

            match result {
                Ok(status) => {
                    if !status.success() {
                        println!("Failed to display file.");
                    }
                }
                Err(_) => println!("Failed to execute cat."),
            }
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);
            let result = Command::new("sh").arg("-c").arg(command).status();

            match result {
                Ok(status) => {
                    if status.success() {
                        println!("File '{}' created successfully.", path);
                    } else {
                        println!("Failed to create file.");
                    }
                }
                Err(_) => println!("Failed to execute create command."),
            }
        }

        FileOperation::Remove(path) => {
            let result = Command::new("rm").arg(&path).status();

            match result {
                Ok(status) => {
                    if status.success() {
                        println!("File '{}' removed successfully.", path);
                    } else {
                        println!("Failed to remove file.");
                    }
                }
                Err(_) => println!("Failed to execute rm."),
            }
        }

        FileOperation::Pwd => {
            let result = Command::new("pwd").status();

            match result {
                Ok(status) => {
                    if !status.success() {
                        println!("Failed to print working directory.");
                    }
                }
                Err(_) => println!("Failed to execute pwd."),
            }
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        println!("Enter your choice (0-5):");

        let choice = read_input();

        if choice == "0" {
            println!("Goodbye!");
            break;
        } else if choice == "1" {
            println!("Enter directory path:");
            let path = read_input();
            let operation = FileOperation::List(path);
            perform_operation(operation);
        } else if choice == "2" {
            println!("Enter file path:");
            let path = read_input();
            let operation = FileOperation::Display(path);
            perform_operation(operation);
        } else if choice == "3" {
            println!("Enter file path:");
            let path = read_input();
            println!("Enter content:");
            let content = read_input();
            let operation = FileOperation::Create(path, content);
            perform_operation(operation);
        } else if choice == "4" {
            println!("Enter file path:");
            let path = read_input();
            let operation = FileOperation::Remove(path);
            perform_operation(operation);
        } else if choice == "5" {
            let operation = FileOperation::Pwd;
            perform_operation(operation);
        } else {
            println!("Invalid menu option. Please try again.");
        }
    }
}