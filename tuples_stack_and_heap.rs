fn main () {

    let group = (1, 5, "Oi amor."); //Como tudo esta no stack, é possível fazer uma cópia para group_
    let group_ = group;
    println!("{:?}", group);

    let group1 = (1, 5, String::from("Oi amor.")); //Aqui tem uma variável, a última, que não está no stack, e isso resulta em perdendo o valor de group1
    let group2 = group1.clone();
    println!("{:?}", group1);


}
