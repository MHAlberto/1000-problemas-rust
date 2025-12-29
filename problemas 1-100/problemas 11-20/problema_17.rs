fn main() {
    let empaquetado: (i32, f64, u8) = (10, 3.1415, 255);

    let x = empaquetado.0;
    let y = empaquetado.1;
    let z = empaquetado.2;

    println!("El valor inicial es {x}");
    println!("El valor intemedio es {y}");
    println!("El valor final es {z}");
}
