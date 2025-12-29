fn main() {
    let mut z = 5;
    z = z + 1; // z = 6
    let z = z + 1; // z = 7
    let z = z * 2; // z = 14

    // No podemos descomentar la siguiente linea, porque no es mutable la ultima asginacion.
    // z = z + 1;

    println!("El valor de z = {z}")
}
