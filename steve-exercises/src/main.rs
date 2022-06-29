// 46 exercises meant for python assigned by my programming teacher to his old students.
// I've repurposed them to help me get better at the rust language.

// Reusable functions
fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
    .read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

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






// Exercise 2
// Define a function max_of_three() that takes three numbers as arguments and returns the largest of them.

fn max_of_three(num1: i32, num2: i32, num3: i32) -> i32 {
    if num1 > num2 && num1 > num3 {
        num1
    }
    else if (num2 > num1 && num2 > num3) {
        num2
    }
    else {
        num3
    }
}
 
fn max_of_three_test_case() {
    println!("{}",max_of_three(2,4,6));
    println!("{}",max_of_three(6,2,4));
    println!("{}",max_of_three(1,9,3));
}

// Exercise 3
// Define a function that computes the length of a given list or string.

fn len(word: &str) -> i32 {
    let mut count = 0;
    for i in word.chars() {
        count += 1;
    }
    count

}

fn len_test_case() {
    println!("{}",len("|WAAAAA"));
    println!("{}",len("    "));
    println!("{}",len("1234"));
    println!("{}", len(&input()))
}

// Exercise 4
fn is_vowel(char: char) -> bool{
    let list_of_vowels = ['a','e', 'i', 'o', 'u'];
    for i in list_of_vowels {
        if char == i {
            return true
        }
    }
    return false
}

fn main() {
      
}
