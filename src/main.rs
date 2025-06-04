use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Red")).or_insert(40);
    scores.entry(String::from("Blue")).or_insert(40);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("blue's score is {score}");

    for (key, value) in scores {
        println!("{key}: {value}")
    }
}

