use std::fmt;

#[derive(Debug)]
struct Display(f32, f32);

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Debug{
    real: f32,
    image: f32,
}

impl fmt::Display for Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.real, self.image)
    }
}

fn main(){
    let d = Display(3.3, 7.2);
    println!("{}+{}i", d.0, d.1);

    let debug = Debug{real: 3.3, image: 7.2};
    println!("{:#?}", debug);
}