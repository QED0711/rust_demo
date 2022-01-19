// Define Structs
struct Dog {name: String}
struct Cat {name: String}

// Define traits
trait Pet {
    fn speak(&self);
    fn sleep(&self) {
        println!("zzzzz...");
    }
}

impl Pet for Dog {
    fn speak(&self){
        println!("Woof, my name is {}", self.name);
    }
}

impl Pet for Cat {
    fn speak (&self){
        println!("Meow, my name is {}", self.name);
    }
    fn sleep(&self){
        println!("purrr...");
    }
}

fn main() {

    let dog = Dog{name: "Rover".to_string()};
    dog.speak();
    dog.sleep();
    
    let cat = Cat{name: "Mittens".to_string()};
    cat.speak();
    cat.sleep();
}
