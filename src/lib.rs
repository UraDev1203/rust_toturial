#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn filters_size_42_shoes() {
        let shoes = vec![
            Shoe { size: 42, style: String::from("sneaker") },
            Shoe { size: 43, style: String::from("sandal") },
            Shoe { size: 42, style: String::from("boot") },
        ]; 

        let in_my_size = shoes_in_size(shoes, 42);  

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 42, style: String::from("sneaker") },
                Shoe { size: 42, style: String::from("boot") },
            ]
        );
    }
}