// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age: u8 = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("Surf"),
    };

    println!("Success!");
} 
