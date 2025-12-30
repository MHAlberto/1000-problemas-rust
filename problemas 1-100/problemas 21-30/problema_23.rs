fn main(){
    mostrar_edad("G",25);
}
fn mostrar_edad(nombre:&str,edad:u8){
    println!("La edad de {} es {}",nombre,edad);
}