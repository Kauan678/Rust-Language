use std::io;

fn fibonacci(n: u32) -> u32 {

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {

    println!("Digite os números da sequência:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Falha ao ler a linha!");
    let n: u32 = n.trim().parse().expect("Por favor, digite um número!");

    for i in 0..n {
        println!("{}", fibonacci(i));
    }
}
