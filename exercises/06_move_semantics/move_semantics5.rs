// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;
    println!("1.x = {:?}", x);

    let z = &mut x;
    *z += 1000;
    println!("z = {:?}", z);
    println!("2.x = {:?}", x);

    let y = &mut x;
    *y += 100;
    println!("y = {:?}", y);
    println!("3.x = {:?}", x);

    assert_eq!(x, 1200);
}
