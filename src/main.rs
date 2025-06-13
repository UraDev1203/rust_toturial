

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");  
    
    let result = longest(x, y);
    println!("The longest string is {result}");
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {x} else {y}
}