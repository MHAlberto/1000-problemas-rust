fn procesar_datos(a: u16, b: i64)-> (){
    println!("Suma: {}", (a as i64) + b);
}
fn main() {
    procesar_datos(500u16, 1000i64);
}