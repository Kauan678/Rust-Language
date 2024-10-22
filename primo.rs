use std::io;

fn is_prime(numero: u32) -> bool {
    if numero <= 1 {
        return false;
    }
    for i in 2..numero {
        if numero % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    
    let mut numero = String::new();

    println!("Digite um número:");
    io::stdin().read_line(&mut numero).expect("Falha ao ler o número");

    let numero: u32 = numero.trim().parse().expect("Por favor, digite um número válido");
    println!("Número lido: {}", numero);

    if is_prime(numero) {
        println!("O número {} é um número primo", numero);
    } else {
        println!("O número {} não é um número primo", numero);
    }
}