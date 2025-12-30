fn main() {
    let valor = "1000"; // &str
    let valor: i64 = valor.parse().unwrap(); 
    println!("El valor numérico es: {valor}");
}