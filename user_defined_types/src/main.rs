struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

type Meter = f64;
type KiloMeter = f64;

fn get_kilometers(meter: &Meter) -> KiloMeter {
    meter / 1000.0
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person {
        name: String::from("Jackie"),
        ..avery
    };
    describe(&jackie);

    let bob = Person {
        name: String::from("Bob"),
        ..peter
    };
    describe(&bob);

    let meter = 100.0;
    println!("{} m is {} km", meter, get_kilometers(&meter));

    let m = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);

    let m = PlayerMove::Pass;
    println!("On this turn: {:?}", m);

    let m = PlayerMove::Teleport { x: 1, y: 0 };
    println!("On this turn: {:?}", m);
}
