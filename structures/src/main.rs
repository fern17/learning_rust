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

fn main() {
    let bg = Color {red: 255, green: 70, blue: 50};
    println!("BG: R: {} G: {} B: {}", bg.red, bg.green, bg.blue);
    //bg.blue = 45; // immutable!
    let mut mutable_bg = Color {red: 0, green: 0, blue: 57};
    print_color(&mutable_bg);
    mutable_bg.red = 100;
    print_color(&mutable_bg);

    let red = ColorTuple(255, 0, 0);
    println!("Red is {} {} {}", red.0, red.1, red.2);

    let rectangle = Rectangle{width: 10, height: 30};
    rectangle.print_description();
    println!("Rectangle is square ? {}", rectangle.is_square());
}

fn print_color(c: &Color) {
    println!("Color ({}, {}, {})", c.red, c.green, c.blue);
}


