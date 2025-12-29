const IVA: f64 = 0.16;
fn main() {
    let tasa_dinamica = IVA;
    println!("El impuesto es: {tasa_dinamica}");
    let tasa_dinamica = tasa_dinamica + 0.01;
    println!("El impuesto es: {tasa_dinamica}");
}
