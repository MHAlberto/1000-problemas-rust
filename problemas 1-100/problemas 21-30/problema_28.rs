fn prueba_retorno(n: i32) -> i32 {
    if n > 10 {
        return n;
    }
    n * 2
}

fn main(){
    println!("{}", prueba_retorno(15)); // Imprime 15, no alcanza al n *2
    println!("{}", prueba_retorno(5));  // Imprime 10, porque 5 no es mayor que 10
}