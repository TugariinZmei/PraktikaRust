// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.



fn main() {
    println!("call_me");
}

fn call_me(x: u32) {
    for i in 0..x {
        println!("Ring! Call number {}", i + 1);
    }
}
