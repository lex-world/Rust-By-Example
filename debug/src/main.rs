// @dev Line3:1 to Line 20:1 - is the main documentation from RBE Book

// // The `derive` attribute automatically creates the implementation
// // required to make this `struct` printable with `fmt::Debug`.
// #[derive(Debug)]
// struct Structure(i32);

// // Put a `Structure` inside of the structure `Deep`. Make it printable
// // also.
// #[derive(Debug)]
// struct Deep(Structure);

// fn main(){
//     // Printing with `{:?}` is similar to with `{}`.
//     println!("{:?}", Structure(3));

//     // The problem with `derive` is there is no control over how
//     // the results look. What if I want this to just show a `7`?
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }

#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8
}

fn main(){
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};

    // Pretty print
    println!("{:#?}", peter);
}