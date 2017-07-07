fn main() {
    // Rust is a strongly-typed language, but you already knew that.
    // Unlike Python or JavaScript, statically-typed languages need
    // to know types of value before compiling, or, in layman's term,
    // running. But that's not really correct...

    // Rust code (and those in most statically-typed languages) gets
    // compiled first, then it gets run.
    // Code in interpreted languages like JavaScript just gets run.
    // The interpreter, runtime, virtual machine, or whatever runs the
    // code figures out all the types on the fly, which makes the
    // your life easier, but to the mercy of the runtime.

    // Rust can infer type of a variable by looking ahead to figure out the
    // value binded to that variable, but there will be time when it's
    // too ambiguous that it asks you to just be explicit.

    // Here are all the numeric types:
    // i8 => 8-bit signed integer
    // u8 => 8-bit unsigned integer
    // i32 => 32-bit signed integer (usually a good default)
    // u32 => 32-bit unsigned integer
    // i64 => 64-bit signed integer
    // u64 => 64-bit unsigned integer
    // isize => signed integer with size up to the computer's arch
    // usize => unsigned integer with size up to the computer's arch

    // Extra neat thing: underscore is permitted anywhere in a number value
    // to make reading easier for humans!

    // explicit type annotation
    let num_1: i32 = 100_000;

    // implicit type inferred
    let num_2 = 200;

    let diff: i32 = num_1 - num_2;

    println!("num_1 is {} and num_2 is {}", num_1, num_2);
    println!("The difference of num_1 and num_2 is {}", diff);

    // floating-point types (decimal numbers)
    // f32 => 32-bit floating-point number
    // f64 => 64-bit floating-point number

    let num_4: f32 = 3.1416;
    let num_5 = 91.231801;

    let sum = num_4 + num_5;

    println!("num_4 is {} and num_5 is {}", num_4, num_5);
    println!("The sum of num_4 and num_5 is {}", sum);

    // boolean type (logical yes/no, true/false)
    let rainy = false;

    // We will touch conditions later :)
    if !rainy {
        println!("Today is a good day");
    }

    // Rust supports primitive characters by using
    // a pair of single quotes to wrap the character.
    let c = 'z';
    let z = 'ℤ';
    let heart = '❤';
    let thai = 'ก';

    println!("{} {} {} {}", c, z, heart, thai);

    // Rust has two primitive compound types: Tuples and Arrays.
    // Compound types are data types that can group multiple values
    // of different types together.

    // This is a tuple...
    let names = ("Pan", "Jon", "Mindy");

    // It can contains different types too
    let random = ('z', 12.1, "Hello");

    // Here is how you access elements in a tuple by its index number.
    // Index starts at 0, not 1.
    // i.e. First element in a tuple has an index number of 0.
    println!("The first name of names is {}", names.0);
    println!("The second value of random is {}", random.1);

    // Tuple is crucial to the concept of pattern-matching, which is
    // a pretty cool programming feature. Basically, pattern-matching
    // lets you do something like this:

    let (first, second, third) = names;
    println!("{}, {}, and {} are coming to the partey!", first, second, third);

    // Array is a different beast. It requires all values in it to have the
    // same data type. Array also has fixed length, which means that it can
    // never shrink or expand.
    let friends = ["Michelle", "Wesley", "Brennen", "Parker", "Kate"];

    // You have a different way to access an element in an array

    let third_friend = friends[2];
    println!("The third friend is {}", third_friend);
}
