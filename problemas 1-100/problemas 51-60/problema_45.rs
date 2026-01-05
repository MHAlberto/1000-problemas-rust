fn main() {
    'externo: loop {
        println!("Entrando al bucle externo");
        loop {
            println!("Entrando al bucle interno");
            let j = 1;
            if j == 1 {
                break;
            }
        }
        break;
    }
    println!("Salida exitosa");
}
