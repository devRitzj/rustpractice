use std::io;

fn main() {
    println!("Enter your email:"); //Enter user email
    let email = get_input();

    println!("Enter your password:"); //enter password
    let password = get_input();

    let is_registered = check_user_registration(&email, &password); //check email

    // Process the result
    match is_registered {
        true => println!("User is registered."),
        false => println!("User is not registered."),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn check_user_registration(email: &str, password: &str) -> bool {
    // Perform the logic to check if the user is registered
    // Replace this with your own implementation or database lookup

    // For demonstration purposes, let's assume there is a registered user with the email "test@example.com"
    let registered_email = "test@example.com";
    let registered_password = "password";

    email == registered_email && password == registered_password
}
