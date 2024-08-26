First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.


New Way to say the 3 rules:

1-Each value in Rust has a variable that's called its owner.
2-There can only be one owner at a time.
3-When the owner goes out of scope, the value will be dropped.

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world");
}

Erro do código acima: Duas variáveis estão com o Hello, quando s2 = hello, a função oculta drop é chamada em s1, e faz com que s1 não tenha mais o "hello".

How to fix that:

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

Output:
   Compiling branches v0.1.0 (C:\Users\T-GAMER\projects\branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target\debug\branches.exe`
s1 = hello, s2 = hello

But, the question is, WHY THIS CODE BELOW IS VALID??

fn main() {
    let x = 5;
    let y = x;

    println!("x is {x}, and y is {y}");
}
Output:
x is 5, and y is 5


The answer is:

But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.

The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.

Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.







