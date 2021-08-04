// Create an interface
trait Shape {
    fn draw(&self);
}

// Create concrete classes to implement the interface
struct Circle();
struct Square();
struct Rectangle();

impl Shape for Circle {
    fn draw(&self) {
        println!("Inside Circle::draw() method.");
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("Inside Square::draw() method.");
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Inside Rectangle::draw() method.");
    }
}

// create a factory to generate object of concrete class based on given info
struct ShapeFactory;
impl ShapeFactory {
    fn new(shape_type: &str) -> Box<dyn Shape> {
        if shape_type == "circle" {
            return Box::new(Circle());
        } else if shape_type == "square" {
            return Box::new(Square());
        } else if shape_type == "rectangle" {
            return Box::new(Rectangle());
        } else {
            return Box::new(Circle());
        }
    }
}

// Use the Factory to get object of concrete class by passing type information
fn main() {
    let shape1: Box<dyn Shape> = ShapeFactory::new("circle");
    shape1.draw();
    let shape2: Box<dyn Shape> = ShapeFactory::new("square");
    shape2.draw();
    let shape3: Box<dyn Shape> = ShapeFactory::new("rectangle");
    shape3.draw();
}
