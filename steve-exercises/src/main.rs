// 46 exercises meant for python assigned by my programming teacher to his old students.
// I've repurposed them to help me get better at the rust language.


// Exercise 1
// Define a function my_max() that takes two numbers as arguments 
// and returns the largest of them. Use the if-then-else construct
fn my_max(num1: i32, num2: i32) -> i32 {
    if num1 > num2 {
        num1
    }
    else {
        num2
    }
}

fn my_max_test_case() {
    println!("{}", my_max(2,4));
    println!("{}", my_max(55, 2));
    println!("{}", my_max(-25, 40));
}

fn main() {
    my_max_test_case()
}
