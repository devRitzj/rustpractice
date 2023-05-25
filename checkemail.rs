use std::io;

fn main() {

    let user_db = vec!["user1@email.com", "user2@email.com", "user3@email.com"];

    println!("Enter your email:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read input");
    let email = email.trim();

    // check email format
    if !validate_email(email) {
        println!("Invalid email format");
        return;
    }

    // Prompt for password
    println!("Enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read input");
    let password = password.trim();

    // Check if user is valid
    match user_db.iter().find(|&user| *user == email) {
        Some(_) => {
            // check password for valid user
            if password == "password123" {
                println!("Login successful!");
            } else {
                println!("Invalid password");
            }
        }
        None => {
            println!("User not registered");
        }
    }
}

fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains(".com")
}

