fn main () {
    let mut arr = [1, 2, 3, 4, 5];
    let slice_mut = &mut arr[1..4];

    slice_mut[0] = 25;

    println!("Array modificado: {:?}", arr);

    let s = String::from("Hello, world!");

    let slice = &s[0..5];
    println!("Slice da string: {}", slice);
}
