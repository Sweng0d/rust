fn main () {

    let mut group1 = (1, 5, String::from("Oi amor.")); //Aqui tem uma variável, a última, que não está no stack, e isso resulta em perdendo o valor de group1
    println!("{:?}", group1);

    //Quero entender a diferença entre clone e referência
    let mut group2 = group1.clone();
    group2 = what_has_here(group2);

    let mut new_group1 = add_number_8_to_tuple(group1);
    println!("The new group1 is {:?}", new_group1);

    println!("Group2 keep being {:?}", group2);

}

fn what_has_here (tuple: (i32, i32, String)) -> (i32, i32, String) {
    println!("Os valores da tuple chamada são {:?}", tuple);
    tuple
}

fn add_number_8_to_tuple (tuple: (i32, i32, String)) -> (i32, i32, String, i32) {
    (tuple.0, tuple.1, tuple.2, 8)
}
