// Create an interface
enum Shape {
    Circle(Circle),
    Square(Square),
    Rectangle(Rectangle),
}

impl Shape {
    fn draw(self) {
        match self {
            Shape::Circle(circle) => circle.draw(),
            Shape::Square(square) => square.draw(),
            Shape::Rectangle(rectangle) => rectangle.draw(),
        }
    }
}
// Create concrete classes to implement the interface
struct Circle();
struct Square();
struct Rectangle();

impl Circle {
    fn draw(self) {
        println!("Inside Circle::draw() method.");
    }
}

impl Square {
    fn draw(self) {
        println!("Inside Square::draw() method.");
    }
}

impl Rectangle {
    fn draw(self) {
        println!("Inside Rectangle::draw() method.");
    }
}

// create a factory to generate object of concrete class based on given info
struct ShapeFactory;
impl ShapeFactory {
    fn new(shape_type: &str) -> Shape {
        if shape_type == "circle" {
            return Shape::Circle(Circle());
        } else if shape_type == "square" {
            return Shape::Square(Square());
        } else if shape_type == "rectangle" {
            return Shape::Rectangle(Rectangle());
        } else {
            return Shape::Circle(Circle());
        }
    }
}

// Use the Factory to get object of concrete class by passing type information
fn main() {
    let shape1: Shape = ShapeFactory::new("circle");
    shape1.draw();
    let shape2: Shape = ShapeFactory::new("square");
    shape2.draw();
    let shape3: Shape = ShapeFactory::new("rectangle");
    shape3.draw();
}
