Temos essa função:

fn main() {
    let heap = String::from("Heyyyy");

    something(heap);
    println!("heap is {}", heap);

}

fn something(param_a: String) {
    println!("{}", param_a);
}

Porém, ao passar heap dentro da função something, temos um problema de owning.
Nesse caso, temos duas opções:

Primeira -> Fazer uma cópia da variável:

fn main() {
    let heap = String::from("Heyyyy");

    something(&heap);
    println!("heap is {}", heap);

}

fn something(param_a: &String) {
    println!("{}", param_a);
}

Segunda -> Passar o valor na função e depois retornar para a variável:

fn main() {
    let heap = String::from("Heyyyy");

    let heap = something(heap);
    println!("heap is {}", heap);

}

fn something(param_a: String) -> String {
    println!("{}", param_a);
    param_a
}

