fn main() {
    let  total_horas = 4;
    println!("O funcionário trabalhou {} horas", total_horas);
    {
        let total_horas = 8;
        println!("O funcionário trabalhou {} horas", total_horas);
    }
    println!("O funcionário trabalhou {} horas", total_horas);
    let total_horas = "Oito";
    println!("O funcionário trabalhou {} horas", total_horas);
}
