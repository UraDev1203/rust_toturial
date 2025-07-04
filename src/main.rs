fn main() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> =  v1.iter().map(|x| x + 1).collect();

    let v1_iter = v2.iter();

    for val in v1_iter {
        println!("Got: {}", val);
        
    }
}
