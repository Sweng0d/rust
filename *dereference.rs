
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(1);      // update this line, don't change other lines!
    assert_eq!(*y, 1);

    *y = 4;
    assert_eq!(*y, 4);
    
    assert_eq!(*x, 5);

    println!("Success!");
}
