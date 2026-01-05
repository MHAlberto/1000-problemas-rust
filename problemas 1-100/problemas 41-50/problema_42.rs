fn main() {
    let temperatura = 35;

    let estado = if temperatura > 30 {
        "Caliente"
    } else {
        "Normal"
    };

    println!("El clima está: {estado}");
}
