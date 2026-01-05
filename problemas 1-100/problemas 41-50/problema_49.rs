fn main() {
    let n = 5;

    if n % 5 == 0 && n % 3 == 0 {
        println!("Divisible por ambos");
    } else if n % 3 == 0 {
        println!("Divisble por tres");
    } else if n % 5 == 0 {
        println!("Divisble por cinco");
    } else {
        println!("No es divisible por ninguno");
    }
}
