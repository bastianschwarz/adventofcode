mod loop_over_lines;
mod array_reduce;

fn main() {
    println!("Loop over lines");
    loop_over_lines::run();

    println!("Array reduce");
    array_reduce::run();
}
