use std::io;

fn fatorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    return n * fatorial(n - 1);
}

fn main() {

    println!("Digite um número para calcular o fatorial:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Falha ao ler a entrada");
    let n: u64 = n.trim().parse().expect("Por favor, digite um número válido!");

    let result = fatorial(n);
    println!("O fatorial de {} é {}", n, result);

}
