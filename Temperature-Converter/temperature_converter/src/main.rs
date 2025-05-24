use std::io;

fn main(){
    println!("Temperature Converter");
    println!("Type 1 for Celsius to Fahrenheit");
    println!("Type 2 for Fahrenheit to Celsius");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Choice: Enter a num (1 or 2)");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("Invalid Choice: Enter 1 or 2");
    }
    
}


fn celsius_to_fahrenheit(){
    println!("Enter Temperature in celsius :->");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input. Please enter a valid number");
            return;
        }
    };

    let fahrenheit = (temp * 9.0/5.0) + 32.0;
    println!("{:.2} C is {:.2} F", temp, fahrenheit);
}

fn fahrenheit_to_celsius(){
    println!("Enter Temperature in Fahrenheit :->");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input, Please enter a valid number");
            return;
        }
    };

    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2} F is {:.2} C", temp, celsius);

}