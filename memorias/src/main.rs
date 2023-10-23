static _ANO: u16 = 2023; // Armazenado em STATIC memory
fn main() {
    let nome = "Enzo"; // Armazenado em STACK memory
    println!("Estamos no ano de {}", _ANO);
    println!("Eu sou {}", nome);
    let users = get_users(); // Função ficticia para buscar dados de usuários em um Banco de Dados
    // users está armazenado em HEAP memory, pois é dinâmico
}
