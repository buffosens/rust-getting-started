// mod common_concepts;

fn main() {
        println!("This is main!");

        let rect1 = Rectangle { width: 30, height: 50 };

        println!(
                "The area of the rectangle is {} square pixels.",
                rect1.area()
        );

        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };
        let rect4 = Rectangle::square(3);

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect4));
}