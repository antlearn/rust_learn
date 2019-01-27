fn plus1(a: i32) -> i32 {
    a + 1
}

fn five() -> i32 {
    5
}

/* So we’re doing something complicated here, long enough that we need
** multiple lines of comments to do it! Whew! Hopefully, this comment will
*/
// explain what’s going on.
fn main() {
    let x = five();
    let y = plus1(x);

    if y % 4 == 0 {
        println!("number is divisible by 4");
    } else if y % 3 == 0 {
        println!("number is divisible by 3");
    } else if y % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let a = [x, y];
    for b in a.iter() {
        print!("  {}", b);
    }
}
