
fn test(a: i32, operator: char ,b: i32) {
    match operator {
            '+' => {
                println!("{}", a + b);
            },
            '-' => {ls
                
                println!("{}", a - b);
            },
            '*' => {
                println!("{}", a * b);
            },
            '/' => {
                if b == 0{
                    println!("Division by 0 is undefined");
                }
                else {
                    println!("{}", a / b);
                }
            },
            '%' => {
                println!("{}", a % b);
            },
            _ => println!("{}","invalid operator"),
        }
}
