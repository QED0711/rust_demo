fn main() {
    struct Point<T> { // define a struct that can have as values some generic type `T`
        x: T,
        y: T,
    }

    impl<T> Point<T> { // Implement methods on the struct, also indicating that the implementations will deal with generic types
        fn x(&self) -> &T { &self.x }
        fn y(&self) -> &T { &self.y }
    }

    // Instantiate
    let p1 = Point { x: 1, y: 5 };
    let p2 = Point { x: 1.2, y: 8.7 };

    // call methods
    println!("{}", p1.x());
    println!("{}", p2.y());

}
