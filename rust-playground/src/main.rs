#![deny(clippy::all)]

//  Ternary Operator
fn main() {
    let defauld_struct = Derived::default();
    println!("default_struct : {:?}", defauld_struct);

    let almost_default_struct = Derived {
        x: 0,
        y: "Y is set".into(),
        ..Default::default()
    };
    println!("almost_default_struct : {:?}", almost_default_struct);

    let nothing: Option<Derived> = None;
    println!("nothing : {:#?}", nothing);
}

#[derive(Debug)]
struct Implemented(String);

#[derive(Debug, Default)]
struct Derived {
    x: u8,
    y: String,
    z: Implemented,
}

impl Default for Implemented {
    fn default() -> Self {
        Self("default".into())
    }
}
