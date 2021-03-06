use crate::global::{int_input, input};

pub fn addition() {
    loop 
    {
        println!("Please input a number!");
        let num1 = int_input();
        println!("Please input another number");
        let num2 = int_input();
        println!("Your result is {}", num1 + num2);
        println!("Would you like to do the operation again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => return,
        }
    }
}

pub fn subtraction() {
    loop 
    {
        println!("Please input a number!");
        let num1 = int_input();
        println!("Please input another number");
        let num2 = int_input();
        println!("Your result is {}", num1 - num2);
        println!("Would you like to do the operation again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => return,
        }
    }
}


pub fn multiply() {
    loop 
    {
        println!("Please input a number!");
        let num1 = int_input();
        println!("Please input another number");
        let num2 = int_input();
        println!("Your result is {}", num1 * num2);
        println!("Would you like to do the operation again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => return,
        }
    }
}

pub fn divide() {
    loop 
    {
        println!("Please input a number!");
        let num1 = int_input();
        println!("Please input another number");
        let num2 = int_input();
        println!("Your result is {}", num1 as f32 / num2 as f32);
        println!("Would you like to do the operation again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => return,
        }
    }
}