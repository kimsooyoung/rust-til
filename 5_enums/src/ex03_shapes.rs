use crate::shapes::Shapes;

pub fn run() {
    let arbitrary_shape = Shapes::Circle {
        radius: 10.0,
        center: (0.0, 0.0),
    };
    let _rect = Shapes::Rectangle {
        width: 10.0,
        height: 20.0,
    };

    let shape_area = arbitrary_shape.area();
    match arbitrary_shape {
        Shapes::Circle { radius, center } => {
            println!(
                "Circle: radius: {}, center: {:?}, area: {}",
                radius, center, shape_area
            );
        }
        Shapes::Rectangle { width, height } => {
            println!(
                "Rectangle: width: {}, height: {}, area: {}",
                width, height, shape_area
            );
        }
    }
}
