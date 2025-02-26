#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

}


fn main() {
    println!("Hello, world!");

    let rect_1 = Rectangle {
        width: 30,
        height: 50
    };

    // println!("rect is {}", rect_1);
    println!("rect1 is {rect_1:#?}");
    dbg!(&rect_1);
    let area_1 = rect_1.area();
    println!("rect1 is {area_1}");

    let area1 = rect_1.area();
    let area2 = Rectangle::area(&rect_1);
    assert_eq!(area1, area2);
}
