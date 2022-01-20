struct Point<T> {x: T, y: T}

trait PointTraits<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
    // fn distance_to(&self, p: &Point<T>);
}

impl PointTraits<f32> for Point<f32> {
    fn x(&self) -> &f32 { &self.x }
    fn y(&self) -> &f32 { &self.y }
}

impl PointTraits<f32> for Point<i32> {
    fn x(&self) -> &f32 { &(*&self.x as f32) }
    fn y(&self) -> &f32 { &(*&self.y as f32) }
    // fn y(&self) -> &f32 { let res = self.y as f32; &res }
}




fn main() {

}
