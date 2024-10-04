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


