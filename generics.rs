//generics

//nem tudo precisa ser genérico
//da para ter váriso tipos de genéricos diferentes na mesma struct
struct Point<T, U> {
    x: T,
    y: U,
}

struct DougsData<T> {
    x: i32,
    y: T,
    z: T
}
fn main() {
    let a = Point {
        x: 100,
        y: -1_f32,
    };
    println!("x = {}, y = {}", a.x, a.y);

    let b = Point {
        x: 10.1,
        y: -2.3,
    };
    println!("x = {}, y = {}", b.x, b.y);
}

