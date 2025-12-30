fn main() {
    let x = 5;
    let y = {
        let x = 3;
        // x + 1;  No hace nada porque no se asigna a nada
        x + 5
    };
    println!("El valor de y es: {y}"); // y = 8
}