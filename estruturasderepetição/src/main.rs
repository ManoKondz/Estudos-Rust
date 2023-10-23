fn main() {
    let mut count = 0;
    while count < 5 {
        println!("Olá Mundo");
        count += 1;
    }
    let lista_de_compras = &["maçãs", "bananas", "café", "laranjas", "goiaba"];
    for count in lista_de_compras {
        println!("Eu gosto de {}", count);
    }
    while true {
        println!("Count = {count}");
        if count == 10 {
            break;
        }
        count += 1;
    }
}
