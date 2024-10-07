Char → 

Em Rust, `char` é um tipo de dado que representa um único caractere Unicode, podendo incluir letras, números, símbolos, ou até mesmo emojis. Cada `char` em Rust ocupa 4 bytes e suporta caracteres de várias línguas e sistemas de escrita, diferentemente de tipos mais limitados como `char` em outras linguagens, que geralmente representam apenas caracteres ASCII. Por exemplo, você pode declarar um caractere assim


Em Rust, a diferença entre `"` e `'` é:

- **Aspas duplas (`"`)**: São usadas para criar **strings** (`&str`), que são sequências de caracteres. Exemplo: `"Hello, world!"`.
- **Aspas simples (`'`)**: São usadas para definir um **char**, que representa um único caractere Unicode. Exemplo: `'a'`.

Então, use aspas simples para caracteres únicos e aspas duplas para strings.

O **unit type** em Rust é representado como `()`. Ele indica a ausência de valor ou o fato de que uma função não retorna nada significativo. É semelhante ao `void` em outras linguagens de programação. É usado, por exemplo, em funções que não têm um valor de retorno explícito

let _v: () = ();

A diferença entre o **unit type** (`()`) e uma **tuple** (tupla) em Rust é:

- **Unit (`()`)**: Representa um tipo vazio ou um valor sem dados. É um tipo especial com apenas um valor possível: `()`. Ele indica a ausência de informação, similar ao `void` em outras linguagens.
- **Tuple**: Pode conter múltiplos valores de diferentes tipos. Exemplo: `(i32, f64, bool)`. Tuplas podem armazenar dados e têm tamanhos e tipos fixos definidos durante a criação.

O unit é essencialmente uma tupla vazia.

Char → 4 bytes

Bool → 1 byte

Unit→ 0bytes

Statement:

let x = 20;

### 1. **Statements**:

- **Executam ações** e **não retornam valores**.
- Exemplo: `let x = 10;` é um statement porque ele cria a variável `x`, mas não retorna nada que possa ser usado em outra parte do código.

### 2. **Expressions**:

- **Avaliam e retornam um valor**.
- Exemplo: `5 + 3` ou o bloco `{ let y = 2; y + 1 }` são expressões que resultam em um valor (`8` e `3`, respectivamente).

**Resumo**: Statements fazem algo, mas não têm valor. Expressões têm um valor que pode ser usado.

All funcions input NEED to ser selecionados os tipos do que será inserido 

fn sum (x: i32, y:i32) → i32 {

x + y

}

panic!() → macro que para estudo e retorna um erro

Stack 

-Fast memory creation and retrieval… Speed, Speed, Speed

-Memory is automatically recaptured by the program after variables go out of scope

-Stack é o default em Rust

-Fixed size variables

let stack_i8: i8 = 10;

let stack_f32: f32 = 20;

let stack_bool: bool = true;

let stack_char: chart = ‘a’;

Rust sabe exatamente o tamanho que deixar para cada variável

Heap

-Flexibility

-Memory that can grow in size (Vector, Hashmap, String, etc)

-Runtime performance cost (speed)

-Memory that can live beyond the scope that created it

-Memory is automatically recaptured when the last OWNER goes out of scope

let heap_vector: Vec<i8> = Vec::new(); //vec![5,2];

let heap_string: String = String::from(”Howdy”);

let heap_i8: Box<i8> = Box::new(30);

Now, the difference in practice:

let stack_i8_2 = stack_i8;

println!(”{}”, stack_i8);

println!(”{}”, stack_i8_2);

//no erros

let heap_i8_2= heap_i8;

println!(”{}”, heap_i8);

println!(”{}”, heap_i8_2);

//error!!

We can fix it:

let heap_i8_2 = &heap_i8; 

or

let heap_i8_2 = heap_i8.clone();

Isso é literalmente fazer uma cópia, em outras linguagens 2 variáveis podem apontar para a mesma memória, em Rust não.

This works:
fn main() {
    let stack = 10;
    let stack2 = stack;

    println!("stack is {}", stack);
    println!("stack2 is {}", stack2);
}

- `&str` é uma **referência imutável** para uma sequência de caracteres. Quando você usa uma string literal, como `"hello"`, você está criando uma `&str` que é alocada em uma região estática da memória.
- **Copiar uma `&str`** significa apenas copiar a referência, e não o conteúdo da string. Em Rust, tipos como referências (`&str`), inteiros (`i32`), e outros tipos primitivos implementam o trait `Copy`, o que permite que sejam copiados sem mover a propriedade.
- Como resultado, você pode ter múltiplas referências imutáveis (`&str`) para o mesmo valor, e isso não entra em conflito com as regras de ownership ou borrowing.

let s1 = "hello"; // `s1` é uma `&str` estática
let s2 = s1;      // `s2` é uma cópia da referência imutável de `s1`

println!("{}", s1); // Funciona, pois `&str` é `Copy`
println!("{}", s2); // Funciona também

### **`String` – Tipo de Dados para Strings Alocadas Dinamicamente**

- `String` é um tipo de dados proprietário que possui a sequência de caracteres. Quando você cria um `String`, ele aloca memória no heap para armazenar seu conteúdo e mantém controle sobre essa memória.
- Diferente de `&str`, o tipo `String` não implementa o trait `Copy`. Em vez disso, ele implementa o trait `Drop` porque precisa liberar a memória quando sai do escopo.
- Quando você faz uma atribuição como `let s2 = s1;`, você está **movendo** a propriedade do `String` de `s1` para `s2`. Isso significa que `s1` se torna inválido e não pode mais ser usado.

let s1 = String::from("hello");
let s2 = s1; // `s1` é movido para `s2`

println!("{}", s1); // Erro! `s1` não é mais válido
println!("{}", s2); // Funciona

Three Rules of Ownership

1-Each value in Rust has an owner

2- There can only be one owner at a time

3- When the owner goes out of scope, the value will be dropped

STACK → COPY

HEAP → MOVE

Se você vai colocar uma referencia como input de uma função isso deve ser notificado

// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}


