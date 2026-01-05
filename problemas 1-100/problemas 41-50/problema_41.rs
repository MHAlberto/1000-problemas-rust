fn main() {
    let stock = 10;

    if stock {
        println!("Tenemos productos disponibles"); //expected `bool`, found integer
    }
}
