fn area_cuadrado(lado:i32)->i32{
    lado * lado
}

fn main(){
    let resultado = area_cuadrado(4);
    assert_eq!(resultado, 16);
}