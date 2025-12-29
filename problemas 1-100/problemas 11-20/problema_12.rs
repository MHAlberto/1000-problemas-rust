fn main() {
    let mut byte: u8 = 255;
    byte = byte + 1;
    println!("El valor del byte = {byte}")
}
/*
Al ejecutar: cargo run --release
Nuestro código nos regresa:

El valor del byte = 0
*/
