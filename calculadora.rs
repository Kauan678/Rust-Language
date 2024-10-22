use std::io;

fn adicao() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("Falha ao ler o número");

    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("Falha ao ler o número");

    let num1: f32 = num1.trim().parse().expect("Falha ao converter o número");
    let num2: f32 = num2.trim().parse().expect("Falha ao converter o número");

    let resultado = num1 + num2;

    println!("O resultado é: {}", resultado);
}

fn subtracao() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("Falha ao ler o número");

    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("Falha ao ler o número");

    let num1: f32 = num1.trim().parse().expect("Falha ao converter o número");
    let num2: f32 = num2.trim().parse().expect("Falha ao converter o número");

    let resultado = num1 - num2;

    println!("O resultado é: {}", resultado);
}

fn multiplicacao() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("Falha ao ler o número");

    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("Falha ao ler o número");

    let num1: f32 = num1.trim().parse().expect("Falha ao converter o número");
    let num2: f32 = num2.trim().parse().expect("Falha ao converter o número");

    let resultado = num1 * num2;

    println!("O resultado é: {}", resultado);
}

fn divisao() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("Falha ao ler o número");

    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("Falha ao ler o número");

    let num1: f32 = num1.trim().parse().expect("Falha ao converter o número");
    let num2: f32 = num2.trim().parse().expect("Falha ao converter o número");

    let resultado = num1 / num2;

    println!("O resultado é: {}", resultado);
}

fn main() {
    loop {
        println!("Escolha a operação:");
        println!("1 - Adição");
        println!("2 - Subtração");
        println!("3 - Multiplicação");
        println!("4 - Divisão");
        println!("0 - Sair");

        let mut operacao = String::new();

        io::stdin().read_line(&mut operacao).expect("Falha ao ler a opção");

        let operacao: u32 = operacao.trim().parse().expect("Falha ao converter a opção");

        match operacao {
            1 => adicao(),
            2 => subtracao(),
            3 => multiplicacao(),
            4 => divisao(),
            0 => break,
            _ => println!("Opção inválida"),
        }
    }
}
