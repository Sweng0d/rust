O primeiro tem que ser um String e o resto &str


// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = "world!";
    let s3 = s1 + s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}
