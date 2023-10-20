
fn main() {
    let mut numero1 = String::new();
    let mut numero2 = String::new();

    println!("Ingrese el primer numero: ");
    std::io::stdin().read_line(&mut numero1)
        .expect("Failed to read line");

    println!("Ingrese el segundo numero: ");
    std::io::stdin().read_line(&mut numero2)
        .expect("Failed to read line");

    let numero1: i32 = numero1.trim().parse().expect("Please enter a valid number");
    let numero2: i32 = numero2.trim().parse().expect("Please enter a valid number");

    let suma = numero1 + numero2;
    println!("La suma es: {}", suma);
}