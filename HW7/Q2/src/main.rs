use std::f64::consts::PI;

struct Polygon {
    num_sides: u64,
    side_length: f64,
}
trait RegularPolygon {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64;
    fn apothem(&self) -> f64;
    
}

impl RegularPolygon for Polygon {
    fn perimeter(&self) -> f64 {
        return self.num_sides as f64 * self.side_length
    }
    fn area(&self) -> f64 {
        0.5 * self.perimeter() * self.apothem()
    }
    fn radius(&self) -> f64 {
        self.side_length / (2.0 * PI / self.num_sides as f64).sin()
    }
    fn apothem(&self) -> f64 {
        self.side_length / (2.0 * (PI / self.num_sides as f64).tan())
    }
}

fn main() {
    let num_sides_list = [6, 12, 24, 128, 256, 512, 1024, 2048, 65536];
    
    //let radius_list = [5.0, 10.0, 15.0];
    let radius1 = 5.0;
    
    for &num_sides in &num_sides_list {
        //for &radius in &radius1 { 
            let polygon = Polygon {
                num_sides: num_sides,
                side_length: radius1 //2.0 * radius * (PI / num_sides as f64).tan(),
            };
            let polygon_area = polygon.area();
            let circle_area = PI * polygon.radius() * polygon.radius();

            println!("Number of sides: {}", num_sides);
            println!("Radius: {}", polygon.radius());
            println!("Polygon area: {}", polygon_area);
            println!("Circle area: {}", circle_area);   
            println!("Ratio of Circumscribed to Regular {}", circle_area/polygon_area) 
        //}
    }
}






