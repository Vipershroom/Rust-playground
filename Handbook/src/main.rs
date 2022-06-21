fn main() {
    let string1 = String::from("dksflalk;dfs");
    let string2 = "lsdkfdjngl;ksd";
    println!("{}", longest(string1.as_str(), string2))
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}