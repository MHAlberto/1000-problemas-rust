fn promediar(a:f32,b:f32)->f32{
    (a + b) / 2.0
}
fn main() {
    let a: f32 = 10.0;
    let b: f32 = 20.0;
    println!("Promedio: {}", promediar(a, b));
}