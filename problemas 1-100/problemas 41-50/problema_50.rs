fn main() {
    for numero in 1..7 {
        if numero % 2 == 0 {
            continue;
        }
        println!("Impar: {numero}");
    }
}
