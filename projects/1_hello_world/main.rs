fn main() { // function declaration called 'main'
	println!("Hello, world!"); // Rust macro[1] called 'println'
} // end function declaration

/*
[1]: Macros are delineated by '!' and follow different rules than
functions.
---
Running a Rust program is separate from compiling a program.
compilation has to happen first:
```
	$ rustc main.rs
```

Running the above command will run the Rust compiler and create an
binary executable file. On macOS or Linux this'll show as `main`, and
`main.exe` on Windows. Windows will also include a `main.pdb` file,
which contains debugging information.

Unlike dynamic languages (i.e.: JavaScript, Ruby, Python), Rust is an
ahead-of-time compiled language. (Compare to a just-in-time compiled
language or interpreted language.) In languages like those dynamic
languages mentioned, there a single command to compile and run them.

While this code was compiled using `rustc`, it won't scale. That's
where Cargo will come in handy.

Next folder: ./hello_cargo
 */