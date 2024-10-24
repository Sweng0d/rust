pub trait Descreve {
    fn descreve (&self) -> String;
}

struct Pessoa {
    name: String,
    age: u32,
}

impl Descreve for Pessoa {
    fn descreve (&self) -> String {
        let description: String = format!("The pessoa name is {} and the pessoa age is {}", self.name, self.age);
        description
    }
}

fn main() {
    let pessoa1 = Pessoa {
        name: String::from("Richard"),
        age: 32,
    };

    let pessoa2 = Pessoa {
        name: String::from("Augusto"),
        age: 51,
    };
    
    println!("{}", pessoa1.descreve());
    println!("{}", pessoa2.descreve());
}
