fn main(){
    // type annotated.
    let logical: bool = true;

    // Regular annotation
    let a_float: f64 = 1.0;  

    // Suffix annotation
    let an_integer   = 5i32;

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true; // this returns an error
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
}