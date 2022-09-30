use rand::Rng;
fn main(){
    player1(contador)
    player2(contador2)
    ganador()

}

fn player1() {
    let mut rng = rand::thread_rng();
    let mut jugador: u8 = rng.gen_range(1..3);
    println!("usted es el jugador {}", jugador);
    if jugador == 1 {
        let mut cronos = 0;
        let mut contador = 0;
        loop{
            let mut rng = rand::thread_rng();
            let mut tira: u32 = rng.gen_range(1..11);
            println!("sus pinos caidos son  {}", tira);
            let mut arreglo: [u32; 10 ] = [0; 10];
            println!("su puntaje es {:?}", arreglo);
            cronos += 1;
            contador += tira;
            println!("{}",contador);
            println!("{}", cronos);
            arreglo[cronos] = tira;
            
            if cronos == 10 {
                break
            }
        }

    }
    retum contador;
}
fn player2(){
    let mut rng = rand::thread_rng();
    let mut jugador: u8 = rng.gen_range(1..3);
    println!("usted es el jugador {}", jugador);
    if jugador == 1 {
        let mut cronos = 0;
        let mut contador = 0;
        loop{
            let mut rng = rand::thread_rng();
            let mut tira: u32 = rng.gen_range(1..11);
            println!("sus pinos caidos son  {}", tira);
            let mut arreglo: [u32; 10 ] = [0; 10];
            println!("su puntaje es {:?}", arreglo);
            cronos += 1;
            contador2 += tira;
            println!("{}",contador2);
            println!("{}", cronos);
            arreglo[cronos] = tira;
            
            if cronos == 10 {
                break
            }
            
            
            

        }

    }
    retum contador2;
   
            
            
            

    
fn ganador(){
    if contador > contador2{
        prinln("el ganador uno, gano")
    }
    if contador < contador2{
        prinln("el ganador dos, gano")
    }
    


}

