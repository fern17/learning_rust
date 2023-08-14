struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTuple(u8, u8, u8);

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool{
        self.width == self.height
    }
}

impl ToString for Rectangle {
    fn to_string(&self) -> String {
        return format!("Width: {}, Height {}", self.width, self.height)
    }
}

struct Person {
    name: String,
    age: u8
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

use std::ops::Add;
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T> 
    where T: Add<Output = T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

fn main() {
    {
        let bg = Color {red: 255, green: 70, blue: 50};
        println!("BG: R: {} G: {} B: {}", bg.red, bg.green, bg.blue);
        //bg.blue = 45; // immutable!
        let mut mutable_bg = Color {red: 0, green: 0, blue: 57};
        print_color(&mutable_bg);
        mutable_bg.red = 100;
        print_color(&mutable_bg);

        let red = ColorTuple(255, 0, 0);
        println!("Red is {} {} {}", red.0, red.1, red.2);
    }

    {
        let rectangle = Rectangle{width: 10, height: 30};
        rectangle.print_description();
        println!("Rectangle is square ? {}", rectangle.is_square());
        println!("{}", rectangle.to_string());
    }

    {
        let person1 = Person {name: String::from("Peter"), age: 41};
        let person2 = Person {name: String::from("Bob"), age: 0};
        println!("Can {} speak ? {}", person1.name, person1.can_speak());
        println!("Can {} speak ? {}", person2.name, person2.can_speak());
    }

    let coord1 = Point {x: 5.0, y: 2.0};
    let coord2 = Point {x: 1.0, y: 3.0};
    let sum = coord1 + coord2;
    println!("{:?}", sum);
}

fn print_color(c: &Color) {
    println!("Color ({}, {}, {})", c.red, c.green, c.blue);
}


