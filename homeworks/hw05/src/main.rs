enum Shape {
    Triangle,
    Square,
    Pentagon,
    Line
}

impl Shape {
    fn corners(&self) -> i8{
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            _ => 0
        }
    }
    
}

fn main() {
    let s = Shape::Triangle;
    println!("{}", s.corners());
}
