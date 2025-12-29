/*
Rust infiere que al declarar 5 / 3 son enteros, pero al colocarle un decimal
ya nos regresa un valor que no se trunca.
*/
fn main() {
    let resultado = 5.0 / 3.0;
    println!("El resultado no truncado {resultado}")
}
