fn calcular(n: i32) -> i32 {
    let n = n + 5; // n = 0 + 5 = 5
    let n = n * 2; // n = 5 * 2 = 10
    n // Devuelve 10.
}
fn main() {
    let n = 0;
    println!("El resultado de calcular({n}) es: {}", calcular(n)); 
}