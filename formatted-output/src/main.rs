fn main() {
    print!("Hello, world!");
    println!("Hello, world!");

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.
    println!("{} days", 31i64);

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can pad numbers with extra zeroes.
    // You can right-align text with a specified width. This will output "000001".
    println!("{number:0>width$}", number = 1, width = 6);
    // You can left-align text with a specified width. This will output "100000".
    // "<" - right or ">" - left can be used to specify alignment.
    println!("{number:0<width$}", number = 1, width = 6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Task
    println!("Pi is roughly {number:.prec$}", number = 3.141592, prec = 3);
}
