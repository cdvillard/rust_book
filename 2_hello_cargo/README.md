# Hello Cargo

This Cargo project was started using the `cargo` command line tool:

```bash
    $ cargo new hello_cargo
```

and generated all of the code in this directory save for this README file. It would also generate a `.gitignore` if the command is run in a directory not version controlled by Git.

Cargo helps organize code by expecting it all to live under the `/src` directory. Everything else organizational such as READMEs, configurations, package locks, and otherwise should live at in the top-level directory.

Running the `cargo build` command will create an executable file in the `target` folder, and by default in the `debug` folder. It can be run using:

```bash
    $ ./target/debug/hello_cargo # Hello world!
```

We could also run `cargo run` from within the folder, which will build and run our code all at once.

Cargo is smart enough to recognize if changes have been made to files
and will only compile a new build if it detects said changes.

Another useful command, `cargo check`, let us see if our code will compile without running a full build step.

When a project is in fact ready for release, running the build flag with a `--release` flag will compile your code with optimizations and create a `release` folder within `./target`. When developing code, these optimizations will make compile time longer, but does make the compiled code run faster. That makes the `--release` flag useful for benchmarking code's running time.

Using Cargo on simple projects doesn't provide much value, but it's worth learning the conventions. The following can reliably run most projects:

```bash
    $ git clone example.org/projectname
    $ cd projectname
    $ cargo build
```
