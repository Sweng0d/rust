How to Fix That:

fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
}


You need to:

fn main() {
    assert!(0.1 as f32+0.2 as f32==0.3);

    println!("Success!");
}
