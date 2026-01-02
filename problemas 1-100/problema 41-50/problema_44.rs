fn main() {
    let mut contador = 0;
    let resultado = loop {
        contador += 1;

        if contador == 5 {
            break contador * 10;
        }
    };
    assert_eq!(resultado, 50);
}
