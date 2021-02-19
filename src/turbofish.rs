// What is Rust's turbofish?

// First example
/* fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_numbers = numbers.into_iter().filter(|n| n % 2 == 0).collect();

    // This can be fixed in two differents ways

    // let even_numbers: Vec<i32> = numbers.into_iter().filter(|n| n % 2 == 0).collect();
    // let even_numbers = numbers
    //     .into_iter()
    //     .filter(|n| n % 2 == 0)
    //     .collect::<Vec<i32>>();

    println!("{:?}", even_numbers);
} */

// ------------------------------------------------------------

/* fn main() {
    let s = "Hello world!";
    // let string = s.into();

    let string = s.into::<String>();

    /* Why does ::<> work on collect but not on into?
    The answer is in the type signatures of those two functions. */

    /* collect looks like this:
    fn collect<B>(self) -> B  */

    /* into looks like this:
    fn into(self) -> T */

    /*
    The fact that collect has a generic type <B>
    is what allows you to use ::<>

    into somehow been written as fn into<B> */
} */

// ------------------------------------------------------------

/* fn main() {
    let some = SomeStruct::<String> {
        chave: String::from("valor"),
    };

    println!("{:?}", some);
}
#[derive(Debug)]
struct SomeStruct<T> {
    chave: T,
} */

// ------------------------------------------------------------

/*
// The fully qualified trait syntax:
// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
 */

// ------------------------------------------------------------

/* fn main() {
    let s = "Hello, World!";
    let string = <&str as Into<String>>::into(&s);
    // let string = Into::<String>::into(s);
    println!("{}", string)
} */
