fn main() {
    let nivel = 1;
    {
        let nivel = nivel + 5;
        println!("Nivel interno: {nivel}"); // Resultado es 6
    }
    println!("Nivel externo: {nivel}"); //Resultado es 1
}
