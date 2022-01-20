use std::f64::consts::PI;

struct Rectangle {height: f64, width: f64}
struct Circle {radius: f64}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        &self.height * &self.width
    }
}
impl HasArea for Circle {
    fn area(&self) -> f64 {
        PI * &self.radius.powf(2.0)
    }
}

struct Shape<T: HasArea> (T);
impl<T: HasArea> Shape<T> {
    fn area_times_n(&self, n: f64) -> f64 {
        self.0.area() * n
    }
}


fn main() {

    let rect = Shape(Rectangle{height: 5.5, width: 10.2});
    let circ = Shape(Circle{radius: 3.6});

    println!("{}", rect.area_times_n(5.0));
    println!("{}", circ.area_times_n(5.0));

}
