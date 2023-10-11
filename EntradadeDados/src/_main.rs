use std::io;

fn convert_to_int(data_user: &String) -> i32 {
    let x = data_user.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("Digite o primeiro valor");
    io::stdin().read_line(&mut num1).expect("Erro ao ler o primeiro numero");
    println!("Digite o segundo valor");
    io::stdin().read_line(&mut num2).expect("Erro ao ler o segundo numero");

    if convert_to_int(&num1) > convert_to_int(&num2) {
        println!("O numero {} é maior que {}", num1, num2);
    } else {
        println!("O numero {} e menor ou igual que o numero {}", num1, num2);
    }

}
