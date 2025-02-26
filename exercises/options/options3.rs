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
    // if let默认移动值，除非你用ref借用
    if let Some(ref p) = y{ //将p作为引用绑定到y内的Point，而不是Move
        println!("Co-ordinates are {},{} ", p.x, p.y);
    };
    // match y {
    //     Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
    //     _ => panic!("no match!"),
    // }
    y; // Fix without deleting this line.
}
