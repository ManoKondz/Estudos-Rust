fn main() {
    let idade = 16; // inteiro usando 8bits. O número seguido do 'i' é a quantidade de bits
    let altura = 1.16; // float usando 64 bits. Podendo variar tal qual o inteiro
    let aluno0 = ("Enzo", idade, altura, 'E'); // Tipo composto tupla(array heterogênea)
    println!("O nome do aluno 1 é {}\nSua idade é {}\nSua altura é {}\nSua inicial é {}", aluno0.0, aluno0.1, aluno0.2, aluno0.3);
    let favorite_numbers = [22, 25, 69, 24];
    println!("O número favorito de {} é {}", aluno0.0, favorite_numbers[0]);
}