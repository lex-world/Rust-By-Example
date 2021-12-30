macro_rules! myOwnMacro {
    () => {
        println!("Hello, world!");
    }
}

fn main() {
    myOwnMacro!();
}
