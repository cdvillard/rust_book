/*
    `use` imports libraries. In this case, we're importing
    the input/output (i/o) library from the Rust standard library
    Also importing the `Rng` trait from the `rand` library which
    defines methods that Random Number Generators implment. This
    trait must be in scope for use to use said methods.
    Also importing the 'Ordering' type, an enum with variants 'Less',
    'Greater', and 'Equal'.
*/
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// declaring a function called main
fn main() {
    // Calling the Print Line macro a couple of times
    println!("Guess the number");

    /*
        Implementing 'thread_rng' methods calls a random
        number generator on the current thread of execution
        seeded by the OS. Next, calling 'gen_range', which
        takes a range of intergers as an experession and
        generates a random number within that range. Passing
        in a 'start..=end' expression is inclusive of the lower
        and upper bounds of the range, so we'll get something
        between 1 and 100.
     */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    /*
        Printing the secrent number for testing purposes using
        Rust's placeholder syntax to pass in the variable. Just
        gonna comment this out and use it for testing purposes. ;)
     */
    // println!("The secret number is: {secret_number}.");

    /*
        Wrapping this into a loop will allow for multiple runs of the code
        within the loop block, until a `break` statement is run. In this case,
        we run said break statement if the user matches the secret number.
     */
    loop {
        println!("Please input your guess.");

        /*
            Declaring a variable using `let`, making it a
            mutable variable as variable are immutable by
            default and assigning it the result of String's
            associated function, 'new', which creates a new
            instance of a 'String'. An associated function
            is a function implemented on a type.
        */

        let mut guess = String::new();

        /*
        Implementing the 'stdin' associated function from the `io`
            library we imported in the prelude (at the top). This
            returns an instance of type 'std::io:Stdin`, which
            represents a handle to the standard input of a terminal.
            Then calling `read_line` method from the stdin handler we
            instanciated to get input from the user. Passing in `&mut guess`
            to tell `read_line` where to store the input. `read_line` will
            append that string to a mutable variable without overwriting its
            contents.
            /*!
             * &mut is doing two separate jobs at once.
             *
             * & is indicating a references to a previous variable. In this case,
             * `guess`. This is to allow access to a piece of data without copying
             * it into memory multiple times.
             *
             * `mut` is indicating that we're referring to a reference in a mutable way.
             * This allows us to update the value and override it.
            */
            `read_line` not only accepts the string we pass in, but also returns a value
            of type `Result`, an enumeration (enum). Enums in Rust contain multiple possible
            states called `variants`. `Result` specifically returns `Ok` and `Err` variants,
            representing a successful operation and the generated value or a failed operation
            and information about why it failed respectively.

            The 'Result' type has a callable `expect` method in-built. If the instance of
            Result is an `Err` value, `expect` will crash the program and siplay the message
            passed into it. If `read_line` returns an `Err`, it's likely an OS-level problem.
            But if the instance of `Result` returns an `Ok` value, `expect` will just return
            the generated value from within `Ok` and return that to be used. It's possilbe to
            not use `expect`, except a warning will be thrown at compile time.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //

        /*
            This line helps us convert the `guess` variable we instantiated from type `String
            into an unsigned, 32-bit interger (u32). This is necessary because Rust uses a
            strict, static type system. This means that Rust will not see a string containing
            "32" and guess it should be treated with any equivalence to an interger 32. It will
            simply not compare the two unless the string is converted to an interger or the
            interger to a string.

            Here, we're declaring a variable of `guess` once again, but using a method called
            `shadowing` that allows us to reuse variable names and convert a value from one type
            to another. We're then assigning the new `guess` to the value of the old `guess` with
            some alterations:

                - `guess.trim()` to remove whitespace, newlines, or carriage returns
                - `.parse()` to convert the string to the declared type u32 (guess: u32)

            If `parse` does return a converted number of type u32, Rust will also infer (it does come
            with built-in inference) that `secret_number` is also a u32, and thus will be able to
            compare the numbers.

            Whereas earlier we used `.expect` to handle errors in case `io::stdin` couldn't read the line
            we're using `match` to catch the error and handle it without crashing the program. `parse`
            can return either an `Ok` `Result` variant or an `Err` `Result` variant. If it's `Ok`, then
            we assign the value to `guess`. If it's `Err`, we let the code run to the next iteration of
            loop.
        */
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
            If the `Result` returned from `read_line` is an `Ok` value, we'll run the
            `println` macro one more time, printing the value of `guess` within curly
            braces, known as placeholders.
        */
        println!("You guessed: {guess}");

        /*
            Taking the value inputed into `guess` and using `cmp` to compare to the
            generated secrent number. `cmp` returns a variant of `Ordering`, which
            we compare using a `match` expression to decide what to do next based on
            which variang is returned.

            `match` expressions are made up of `arms`, similar to how JavaScript's
            `switch` statement is made up of `cases`. Each `arm` consists of a
            pattern to match against, and code that should be run should that arm's
            pattern match the value passed into the match expression.

            Walking throuhg the example, if `guess` is Greater than `secret_number`,
            `cmp` will return `Ordering::Greater`. `match` will then compare `cmp`'s
            returned `Ordering::Greater` to each arm of the match expression. It'll
            first check against `Ordering::Less`, which isn't equivalent, so it'll
            ignore the code in the right-side statement of that arm and  go to the
            next arm. The next arm happens to be `Ordering::Greater`, which means it
            is equivalent, and so will run the right-side statement and print "TOO BIG"
            to the terminal. The match expression will end after the first successful
            match, so the last arm is ignored.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("TOO BIG"),
            Ordering::Equal => {
                println!("You win!");
                println!("Let's play again! See ya!");
                break
            },
        }
    }
}
