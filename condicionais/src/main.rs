static NUM_PIVO: i8 = 5;
fn main() {
    let mut comparador = 4;
    if comparador < NUM_PIVO {
        println!("O comparador é menor");
    } else {
        println!("O comparador é maior");
    }
    comparador += 1;
    if comparador < NUM_PIVO {
        println!("O comparador é menor");
    } else {
        println!("O comparador é maior");
    }
    comparador += 1;
    if comparador < NUM_PIVO {
        println!("O comparador é menor");
    } else {
        println!("O comparador é maior");
    }
}
