struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some( ref p) => {
            println!("Co-ordinates are {},{} ", p.x, p.y);
            drop(y); // 使用 drop 函数来显式地放弃对 y 的所有权。
        }
        _ => panic!("no match!"),
    }
}
