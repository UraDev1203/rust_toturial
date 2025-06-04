
fn main() {

    let data = "initial contents";
    let mut s = data.to_string();

    s.push_str("bar");
    println!("{s}");
}
