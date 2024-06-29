const SECONDS_IN_MINUTES: u32 = 60;
const MINUTES_IN_HOURS: u32 = 60;
const SECONDS_IN_HOURS: u32 = SECONDS_IN_MINUTES * MINUTES_IN_HOURS;

fn main() {
    let number1 = 22; // O "let" é a forma de declarar uma variável.
    println!("Trabalhou {} Horas.", number1);

    let mut number2 = 26; // O "mut" é a forma de tornar uma variável mutável. "number1 = 26;" é um erro.
    println!("Trabalhou {} Horas.", number2);

    number2 = 40;
    println!("Trabalhou {} Horas.", number2);

    let number3 = 37; // Aqui nós estamos a redeclarar a variável, o que é permitido.
    println!("Trabalhou {} Horas.", number3);

    {
        let number3 = 30; // Aqui, nós criamos um novo escopo, ele não afetará nada do escopo original.
        println!("Trabalhou {} Horas.", number3);
    }

    let number3 = number3 + 10;
    println!("Trabalhou {} Horas.", number3);

    println!("Ao fim de {} de trabalho, ele trabalhou {} Segundos.",number3, number3 * SECONDS_IN_HOURS)
}
