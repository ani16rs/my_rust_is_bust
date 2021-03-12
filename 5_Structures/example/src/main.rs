#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // INTRO
        let width1 = 30;
        let height1 = 50;

        println!(
            "Area of rectangle is {} sq pixels",
            area(height1, width1)
        );



    // REFACTORING WITH TUPLES
        let rect1 = (30, 50);
        println!(
            "Area of rectangle using tuples is {} sq pixels",
            area_using_tuple(rect1)
        );



    // REFACTORING WITH STRUCTS 
        let rect2 = Rectangle { 
            width: 30, 
            height: 50,
        };
        
        println!(
            "Area of rectangle using struct is {} sq pixels",
            area_using_struct(&rect2)
        );

        println!("rect2: {:?}", rect2);
        println!("rect2: {:#?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_using_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
