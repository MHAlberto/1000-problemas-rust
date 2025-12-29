fn main(){
    let meses = ["Ene","Feb","Mar"];
    let indice = 2;

    let valor = meses[indice];
    println!("El mes es: {}", valor);
}

//Lanzará un panic at runtime ya que el índice está fuera del rango del array. 