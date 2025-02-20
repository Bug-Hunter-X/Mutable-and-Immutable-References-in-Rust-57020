fn main() {
    let mut x = 5;
    { // Create a new scope to limit the lifetime of the mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    }
    println!("x = {}", x); // Output: x = 6
}