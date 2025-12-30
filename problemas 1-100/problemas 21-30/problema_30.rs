fn calcular_minutos(horas:i32)->i32{
    horas * 60
}
fn main(){
    let m = calcular_minutos(2);
    println!("2 horas son {} minutos", m);
}