#![allow(dead_code)]

struct Fooo {
    quax: i32,
    baz: String,
    z: Fuz,
}

struct Fuz {
    zed: i32,
}

struct Point2D {
    x: f64,
    y: f64,
}

fn add_points(a: Point2D, b: Point2D) -> Point2D {
    Point2D {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat,
}

enum Relationship {
    Father,
    Mother,
    Daughter,
    Son,
    Sibling,
    Other(u32),
}

struct Foo {
    a: i32,
}

struct Bar {
    b: Foo,
}

enum Baz {
    VarA(Foo),
    VarB(Bar),
}

//--------------------------------------------
enum Action {
    Drive,
    Turn(Direction),
    Stop,
    Pickup,
}

enum Direction {
    Left,
    Right,
}

fn print_action(a: Action) {
    match a {
        Action::Drive => println!("Drive forward!"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turn left!"),
            Direction::Right => println!("Turn Right!"),
        },
        Action::Pickup => println!("Pick up object!"),
        Action::Stop => println!("Stop!"),
    }
}

fn main() {
    let a = Fooo {
        quax: 10,
        baz: String::from("Hello, World!"),
        z: Fuz { zed: 4 },
    };

    println!("Fooo: quax: {}, baz: {}, z.Fuz.zed: {}", a.quax, a.baz, a.z.zed);

    let p1 = Point2D { x: 10.0, y: 4.0 };
    let p2 = Point2D { x: -2.0, y: 30.5 };

    println!("p1 + p2 x: {}", add_points(p1, p2).x);

    let my_pet = Animal::Dog;
    let other_pet = Animal::Cat;

    assert!(my_pet != other_pet);

    //-----------------------------------------
    let program = vec![
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Drive,
        Action::Pickup,
        Action::Turn(Direction::Left),
        Action::Drive,
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Stop,
    ];
    for action in program {
        print_action(action);
    }
}
