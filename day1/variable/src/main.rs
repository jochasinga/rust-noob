fn main() {
    // `let` keyword declares a new variable.
    // Rust will warn you about any unused variables you declare.
    // Below reads "declare variable `name` of (inferred) type `String` and
    // bind it to the value "Pan".
    
    let name = "Pan";

    // You can declare a variable without binding to a value,
    // but Rust cannot infer or "guess" the type for you, and
    // unlike dynamically-typed languages such as Python or JavaSCript,
    // it requires all types known to it before compiling.
    //
    // Below reads "declare variable `age` of type `i32` (32-bit signed integers)

    let age: i32;

    // If you try to use it, Rust will complain about you trying to use an
    // uninitialized variable. That's fair since you have not given it a value.
    // So let's bind the variable to a number...

    age = 19; // Ok, I know that's too low, but this is educational.

    // Uncomment the two lines below to stop the unused variables warning
    
    // println!("{} is the man!", name);
    //println!("He is about {} years old.", age);

    // `const` is used when you need a constant value that's around for the
    // whole lifetime of your program.
    // To be `const`, you must explicitly annotate the type upfront and bind
    // the variable to a literal value (value that does not change while the
    // program is running.
    // A constant has to follow the C-way of naming variable in all caps.
    // Normally you'd create a constant for something that is an established
    // truth or fact, like time (well, Einstein would argue, but he isn't here).

    const CENTURY: i32 = 100;
    println!("A century has {} years.", CENTURY);

    // A constant has the implication that it's never guarantee to be in the
    // same location in a computer's memory, thus using a pointer to a constant
    // is a duh and pointless (pun-intended).

    // All variables in Rust are default to immutable.
    // Immutability is when you declare a variable, bind a value to it
    // and can never change that variable again (like what a wine stain does
    // to your favorite white summer shirt).

    // let a = 50;
    // a = 40; // get scolded here!

    // Immutability is a very important topic in functional programming.
    // Imagine a program that runs something as serious as an autopilot software
    // for a real airplane. What happens if a global variable (meaning a variable
    // that can be accessed from anywhere in a program) is mutable, and some
    // part of the 1,000,000++ lines of code change that global variable, often
    // without the programmer's intention.

    // To create a variable that is mutable, use `mut` keyword.

    let mut b = 99;
    b = 10;

    println!("b equals to {}", b);

    // Global variables in general are considered bad practice and should be
    // limited since it makes a program's state unpredictable.

}
