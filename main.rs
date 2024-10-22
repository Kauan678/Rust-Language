use std::io;

fn main() {

    let mut numero = String::new();

    println!("Digite um número:");
    io::stdin().read_line(&mut numero).expect("Falha ao ler o número");

    let numero: u32 = numero.trim().parse().expect("Por favor, digite um número válido");
    println!("Número lido: {}", numero);

    if numero % 2 == 0 {
        println!("O número {} é par", numero);
    } else {
        println!("O número {} é ímpar", numero);
    }

}