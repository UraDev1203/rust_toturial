struct Point<X, Y> {
    x: X,
    y: Y,
} 

impl<X1, Y1> Point<X1, Y1>  {
    fn x(&self) -> &X1 {
        &self.x
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}

impl Point<f64, f64>  {
    fn distance(&self) -> f64 {
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}

fn main() {
   
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);

    println!("The largest number is {result}");

    let char_list = vec!['y','t','a','w'];
    
    let result = largest(&char_list);

    println!("The largest char is {result}");

    let p = Point{x:5, y:6};
    println!("p.x = {}", p.x());

    let p = Point{x:3.0, y:4.0};
    println!("Point distance is {}", p.distance());

    let p2 = Point{x: "hello", y: "csss"};
    let p3 = p.mixup(p2);
    println!("mixed value p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}