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

add more
