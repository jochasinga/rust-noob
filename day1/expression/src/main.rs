fn main() {
    // Almost everything in Rust is an expression.
    // A statement is when you state something without
    // a return value. For instance, the line below
    // is a statement printing out the text "Hello, world!"
    
    println!("Hello, world!");

    // The previous print statement does not return any value
    // to the main program. It just creates what is known as
    // a side effect--changing the state of your program. In
    // this case, it "stream" the text to a software process
    // known as stdout (standard output) and thus print out
    // the text on the terminal.

    // Here is a proof that it is really a statement.
    // (Comment it when you had enough fun)
    
    let greeting = println!("Hola!");

    // When you run this code, Rust will warn you about `greeting`
    // being an unused, wasteful variable. That's because nothing
    // ever came out of the print statement!

    // On the contrary, an expression is a statement that DOES
    // return something. (Yes, an expression is a meaningful statement)
    // examples would be `1 + 1`, `"Hello"`, `10 / 2`, etc.
    // Let's try that by creating a variable to accept the returned value...

    let sum = 1 + 1;

    // Yes! `sum` has a value.
    println!("The sum of 1 and 1 is {}", sum);

    // Oh, and anything beginning with `let` or `const` is not an expression.
    // It's a completed assignment of an expression to a variable.
    // The line below is totally impossible, so uncomment to overcome the
    // compiler's tantrum.
    
    let redundant = (let sum = 1 + 1);

    // In Rust, you put a semicolon (`;`) to end a statement.
    // An expression, on the other hand, does not need one.
}
