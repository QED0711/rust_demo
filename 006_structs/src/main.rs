fn main() {
    struct Point { x: f32, y: f32 } // define struct
    
    impl Point { // implement methods on structs
        fn distance_to(&self, p2: &Point) -> f32 { // &self makes the instance and its values accessable
            return ((self.x - p2.x).powf(2.0) + (self.y - p2.y).powf(2.0)).sqrt();
        }
    }

    // instantiate 
    let point1 = Point{x: 3.5, y: 6.2};
    let point2 = Point{x: 5.3, y: 8.9};

    // call method
    let dist = point1.distance_to(&point2);

    println!("Distance: {}", dist);
}
