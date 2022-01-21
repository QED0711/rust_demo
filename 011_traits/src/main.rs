use std::f64::consts::PI;

// Base Struct Setup
struct Rectangle {height: f64, width: f64}
struct Circle {radius: f64}

// Trait defines functionality to be shared amongst various other types
trait Area {
    fn get_area(&self) -> f64;
}

// train implementations
impl Area for Rectangle {
    fn get_area(&self) -> f64 {
        &self.height * &self.width
    }
}
impl Area for Circle {
    fn get_area(&self) -> f64 {
        PI * &self.radius.powf(2.0)
    }
}

struct Shape<T: Area> (T);
impl<T: Area> Shape<T> {
    fn area_times_n(&self, n: f64) -> f64 {
        self.0.get_area() * n
    }
}


fn main() {

    let rect = Shape(Rectangle{height: 5.5, width: 10.2});
    let circ = Shape(Circle{radius: 3.6});

    println!("{}", rect.area_times_n(5.0));
    println!("{}", circ.area_times_n(5.0));

}
