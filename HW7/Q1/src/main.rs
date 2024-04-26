use std::f64::consts:: PI;
enum Shape {
    Triangle(f64, f64, f64),
    Rectangle(f64, f64),
    Circle(f64),
}

fn create_shape(shape:&str, lengths:&[f64]) -> Shape{
   match shape{
      "Triangle" => Shape::Triangle(lengths[0], lengths[1], lengths[2]),
      "Rectangle" => Shape::Rectangle(lengths[0], lengths[1]),
      "Circle" => Shape::Circle(lengths[0]),
      _ => todo!()
   }
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Triangle(a, b, c) => {
                    let s = (a + b + c) / 2.0;
                    return (s * (s - a) * (s - b) * (s - c)).sqrt()
               
            }
            Shape::Rectangle(length, width) => {
                    return length * width;
                
            }
            Shape::Circle(radius) => {
                    return PI * radius * radius;
                
            }
        }
    }

    fn perimeter(&self)-> f64 {
        match *self {
            Shape::Triangle(a,b,c) => {
                    return a + b + c;
            }
            Shape::Rectangle(length, width) => {
                    return(2.0 * length) + (2.0 * width);
            }

            Shape::Circle(radius) => {
                    return 2.0 * PI * radius;
            }
        }
    } 

    fn double_perimeter(&self) -> Shape { 
        match *self {
           Shape::Triangle(a,b,c) => { 
                  return Shape::Triangle(a * 2.0, b * 2.0, c * 2.0);
           }
           Shape::Rectangle(length, width) => {
                    return Shape::Rectangle(length * 2.0, width * 2.0);
           }
           Shape::Circle(radius) => {
                    return Shape::Circle(radius * 2.0);
           }

        }
    }

    fn verify_parameters(self) -> bool{
        match self{
            Shape::Triangle(a,b,c) => {
                    return (a>0.0) && (b>0.0) && (c>0.0) && (a + b > c) && (a + c > b) && (b + c > a);
            }

            Shape::Rectangle(length,width) => {
                    return (length>0.0) && (width>0.0);
            }
            Shape::Circle(radius) => { 
                    return radius>0.0;
            }
        }
    }
}

fn main() {
    let triangle = create_shape("Triangle", &[3.0, 4.0, 5.0]);
    let rectangle = create_shape("Rectangle", &[2.0,3.0]);
    let circle = create_shape("Circle",&[1.0]);

    println!("Area of Triangle: {}", triangle.area());
    println!("Area of Rectangle: {}", rectangle.area());
    println!("Area of Circle: {}", circle.area());
    println!("Double Perimeter of Triangle {}", triangle.double_perimeter().perimeter());
    println!("Double Perimeter of Rectangle {}", rectangle.double_perimeter().perimeter());
    println!("Double Perimeter of Circle {}", circle.double_perimeter().perimeter());

    println!("verify_parameters: {}", triangle.verify_parameters());
    println!("verify_parameter: {}", rectangle.verify_parameters());
    println!("verify_paramters: {}", circle.verify_parameters());
}



