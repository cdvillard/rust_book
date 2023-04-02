// `use` imports libraries. In this case, we're importing
// the input/output (i/o) library from the Rust standard library
use std::io;
use rand::Rng;

// declaring a function called main
fn main() {
    // Calling the Print Line macro a couple of times
    println!("Guess the number");

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

    Just in case `read_line` fails, we introduce

    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //

    /*
     If the `Result` returned from `read_line` is an `Ok` value, we'll run the
     `println` macro one more time, printing the value of `guess` within curly
     braces, known as placeholders.
     */
    println!("You guessed: {guess}");
}
