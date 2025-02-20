fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y
    *y += 1; // Modify x through y
    println!("x = {}", x); // Output: x = 6
}