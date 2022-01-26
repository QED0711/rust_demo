use std::{time::{Instant}};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn is_self(&self, p: &Point) -> bool {
        p.x == self.x && p.y == self.y
    }
    fn distance_to(&self, p: &Point) -> f64 {
        if self.is_self(p) {
            return 0.0;
        }
        let x_delta_sqr = (&self.x - p.x).powf(2.0);
        let y_delta_sqr = (&self.y - p.y).powf(2.0);
        (x_delta_sqr + y_delta_sqr).sqrt()
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut points: Vec<Point> = Vec::new();
    for _i in 0..100_000 {
        points.push(Point{x: rng.gen(), y: rng.gen()});
    }

    let now = Instant::now();

    let distances: Vec<Vec<f64>> = points
        .par_iter()
        .map(|p1| -> Vec<f64> {
            points
                .par_iter()
                .map(|p2| -> f64 { p1.distance_to(p2) })
                .filter(|dist| *dist > 0.0)
                .collect()
        })
        .collect();
    
    let elapsed = now.elapsed();
    println!("Time to calculate {} : {:?}ms", distances.len(), (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() as u64 / 1_000_000));

}
