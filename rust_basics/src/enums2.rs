enum Shape{
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64) 
}

fn calculate_area(shape : Shape) -> f64{
    let ans = match shape{
        Shape :: Circle(rad) => 3.14 * rad * rad,
        Shape :: Rectangle(l, b) => l * b,
        Shape :: Square(s) => s * s
    };

    return ans;
}

fn main(){
    let circle = Shape :: Circle(3.0);
    let rect = Shape :: Rectangle(3.0, 4.0);
    let square = Shape :: Square(5.0);

    let area = calculate_area(circle);
    println!("Area of the circle is : {:.4}", area);
}