#[derive(Debug)]
pub enum Shapes {
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}

impl Shapes {
    pub fn area(&self) -> f64 {
        match self {
            Shapes::Circle { radius, .. } => radius * radius * std::f64::consts::PI,
            Shapes::Rectangle { width, height } => width * height,
        }
    }
}
