// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y { // instead of moving the value out of y, we just borrow it
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
    
    /*
     * Another solution is: "match y { Some (ref p)... }"
     * 
     * From the docs (difference between & and ref) https://doc.rust-lang.org/std/keyword.ref.html#-vs-ref :
     * "&Foo matches different objects than Foo does. Some(ref foo) matches the same objects as Some(foo)."
     */
}
