/*  Hello, Cargo!
  https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

  cargo build                  [build using cargo]
  ./target/debug/hello_cargo   [run executable build]
  cargo run                    [compile/build (if not yet built) and run]
  cargo check                  [check for errors, build without ]
                             
  > cargo saves the result of the build in a target/debug directory
  > using cargo makes all the commands the same no matter which OS you're 
    working on.

  cargo build --relase         [compile with optimization]
  ./target/release/hello_cargo [run executable build]
*/

fn main() {
    println!("Hello, world!");
}
