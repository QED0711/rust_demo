

struct Car {
    mpg: f32, 
    tank_size: f32
}

struct Motercycle {
    mpg: f32, 
    tank_size: f32
}

trait Performance {
    fn calc_max_distance(&self) -> f32;
}

impl Performance for Car {
    fn calc_max_distance(&self) -> f32 {
        self.mpg * self.tank_size  
    }
}

impl Performance for Motercycle {
    fn calc_max_distance(&self) -> f32 {
        self.mpg * self.tank_size  
    }
}

struct Vehicle<T: Performance> {kind: T}
impl Vehicle<Car> {
    fn distance<T: Performance>(&self) -> f32 {
        &self.kind.calc_max_distance()
    }
}

fn main() {
    let civic = Vehicle{kind: Car{mpg: 30.5, tank_size: 11.2}};
    let harley = Vehicle{kind: Motercycle{mpg: 50.7, tank_size: 6.5}};

    civic.distance();


}
