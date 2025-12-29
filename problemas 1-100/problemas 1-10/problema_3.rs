fn main() {
    let dato = "42";
    let dato = dato.len();
    {
        let dato = dato * 2;
        println!("Valor en bloque: {dato}");
    }
    println!("Valor final: {dato}");
}
