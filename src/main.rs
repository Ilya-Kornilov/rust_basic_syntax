fn main() {
    // variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);

    // can't reassign a new value to "x"
    // as there's no "mut" annotaion
    // so "x = 7" is NOT possible

    let x = "Hello";
    // but "shadowing" IS possible
    println!("The value of x is: {}", x);
// -------------------------------------------------------

    // constants are immutable
    // so no "mut" annotations can be used with them
    // we have to immediately assigh a value to consts
    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("Subcribers: {}", SUBSCRIBER_COUNT);
    // we have to explicitly put the const's data type
    // consts can't be returned values from functions
    // as they have to be defined through all the program
// -------------------------------------------------------

    // Scalar Data Types. Main are: int, float, bool, char
        
        // integers
            // i8   i16     i32     i64     i128    isize
            // u8   u16     u32     u64     u128    usize
            // 8-128=number of bits
            // isize and usize depend on 
            // computer architecture (lenght = arch)
        let a = 98_222; // Decimal      = base 10
        let b = 0xff;   // Hexadicimal  = base 16
        let c = 0o77;   // Octal        = base 8
        let d = 0b1111_0000; // Binary  = base 2
        let e = b'A';   // Byte         for u8 only
            // max for u8 is 255 so if add 1 to it
            // we'll get overfloating
            // and 256 will become 0

        println!("Decimal for 98_222      is: {}", a);
        println!("Decimal for 0xff        is: {}", b);
        println!("Decimal for 0o77        is: {}", c);
        println!("Decimal for 0b1111_0000 is: {}", d);
        println!("Decimal for b'A';       is: {}", e);

        // floating-point numbers: f64 and f32
        let f = 2.5;    // implisit data type definition
        let g: f32 = 3.3;    // explisit one
        println!("Floating-point numbers:   {}  and   {}", f, g);
            // if float is "5.0" it'll print "5"

        // math operations: addition, substraction,
        //                  multiplication, division,
        //                  and remainder
        let sum = 5 + 10;
        let difference = 95.5 - 4.3;
        let product = 4 * 30;
        let quotient = 56.7 / 32.2;
        let remainder = 43 % 5;
        
        println!("sum        of 5 + 10      is: {}", sum);            
        println!("difference of 95.5 - 4.3  is: {}", difference);            
        println!("product    of 4 * 30      is: {}", product);            
        println!("quotient   of 56.7 / 32.2 is: {}", quotient);            
        println!("remainder  of 43 % 5      is: {}", remainder);

        // booleans: true and false
        let h = true;
        let j = false;
        println!("Booleans: {} or {}", h, j);

        // characters:
        let k = '😉';
        let l = '🔺';
        let m = '&';
        println!("Charaters: {}, {}, and {}", k, l, m);
// ---------------------------------------------------------------

    // Compound Data Types
        
        // a 'fixed size array' of related data of different types
        let comp = ("Rust Programming", 100_001);
        println!("A compound type: {:?}", comp);
            // pay attention to {:?} for printing a compound
        
        let (string, number) = comp;
        println!("  >> 1st element: {}", string);
        println!("  >> 2nd element: {}", number);
        
        let first_element = comp.0;
        let second_element = comp.1;
        println!("  >> first:  {}", first_element);
        println!("  >> second: {}", second_element);

        // arrays cannot be changed in length
            // to do so use 'vectors'
        let error_codes = [200, 404, 500];
        println!("An array: {:?}", error_codes);

        let not_found = error_codes[1];
        println!("\'Not found\' code: {}", not_found);
            // can't get access to the element which index
            // is out of bounds => error
            // error_codes[5] -> does NOT exist

        let byte = ['z'; 8];
            // will create an array of length 8
            // and assign all elements to 'z' character
            // pay attention to semicolon between 0 and 8
        println!("Another array: {:?}", byte);

    // Controle Flow
        let some_number = 5;
        if some_number > 10 {println!(">> add 1")}          // no need for ";"
        else if some_number < 5 {println!(">> add 1")}      // no need for ";"
        else {println!("Number has not been changed")}      // no need for ";"
                                                    // but they could be used
        let condition = true;
        let variable = if condition {5} else {6};
        println!("Since the condition is {}, the variable equals to {}",
                    condition, variable);

    // Loops
        loop {  // works until the 'break' operator is reached
            println!("LOOP: >> again!");
            break;
        }

        let mut counter = 0;
        let res = loop {
            counter += 1;
            if counter == 10 {
                break counter;
                // returns 'counter' value after breaking out of the loop
            }
        };      // if we get the var value out of the loop, we put ";"
        println!("I'm the result of a loop: {}", res);

        let mut floor = 3;
        while floor != 0 {
            println!{"  >> floor: {}", floor};
            floor -= 1;
        }
        println!(" > End of the \'while\'-loop");

        let collection = [10, 20, 30, 40, 50];
        for element in collection.iter() {
            println!("  >> element: {}", element);
        }
        println!(" > End of a collection");

        for piece in 1..5 {
            // will iterate from 1 up to but NOT including 5
            println!("  >> #{}", piece)
        }
        println!(" > Pieces ended!");

    // Functions
        my_function(); // takes no parameters
                       // does not return any values
        let a_number = yet_another_function(12, 4);
        println!("I'm the result of yet another function: {}", a_number);

}

fn my_function() {
    println!("Hey, I've come from another funciton!");
}

fn yet_another_function(x: i32, y: i32) -> i32 {
    x / y
    // no need for 'return' key word
    // although it can be used;
    // no ";" or other signs after the returning value
}
