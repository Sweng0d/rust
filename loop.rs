The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

As an example, change the src/main.rs file in your loops directory to look like this:

fn main() {
    loop {
        println!("again!");
    }
}

When we run this program, we’ll see again! printed over and over continuously until we stop the program manually. Most terminals support the keyboard shortcut ctrl-c to interrupt a program that is stuck in a continual loop. Give it a try:

The symbol ^C represents where you pressed ctrl-c. You may or may not see the word again! printed after the ^C, depending on where the code was in the loop when it received the interrupt signal.

Fortunately, Rust also provides a way to break out of a loop using code. You can place the break keyword within the loop to tell the program when to stop executing the loop. Recall that we did this in the guessing game in the “Quitting After a Correct Guess” section of Chapter 2 to exit the program when the user won the game by guessing the correct number.


One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:


fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. It’s possible to implement behavior like this using a combination of loop, if, else, and break; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a while loop. In Listing 3-3, we use while to loop the program three times, counting down each time, and then, after the loop, print a message and exit.

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

Looping Through a Collection with for
You can also use the while construct to loop over the elements of a collection, such as an array. For example, the loop in Listing 3-4 prints each element in the array a.

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

As a more concise alternative, you can use a for loop and execute some code for each item in a collection. A for loop looks like the code in Listing 3-5.

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

When we run this code, we’ll see the same output as in Listing 3-4. More importantly, we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

Using the for loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array, as you would with the method used in Listing 3-4.

The safety and conciseness of for loops make them the most commonly used loop construct in Rust. Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a while loop in Listing 3-3, most Rustaceans would use a for loop. The way to do that would be to use a Range, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.

Here’s what the countdown would look like using a for loop and another method we’ve not yet talked about, rev, to reverse the range:

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}



