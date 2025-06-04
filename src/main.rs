
fn main() {

    let data = "initial contents";
    let mut s = data.to_string();
    let s2 = "bar";

    s.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

