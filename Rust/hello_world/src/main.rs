// This function name gives a warning by the rustc linter
fn fooFct() -> i32 {
    let result: i32 = 0;
    result
}

fn main() {
    println!("Hello, world!");

    // This unused variable gives a warning by the rustc linter
    let i = 0;

    // The following line gives a warning by the clippy linter, but not by the rustc linters
    let foo = fooFct();

    println!("foo: {foo}");
}
