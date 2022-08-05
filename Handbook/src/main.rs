struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping {}", self.data)
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Hello, World!")
    };
    println!("Smart pointer created!")
}