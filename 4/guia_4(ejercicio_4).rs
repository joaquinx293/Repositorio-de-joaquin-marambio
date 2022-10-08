use std::io::stdin;
fn main(){
    candidato_1();
   
}
fn candidato_1(){
    let mut arreglo_V : [i32 ; 26] = [0 ; 26];
    let mut arreglo_b : [i32 ; 26] = [0 ; 26];
    let mut suma : [i32 ; 2] = [0 ; 2];
    let mut candidatos : [i32 ; 26] = [1; 26];
    let mut ganador : [i32 ; 2] = [0 ; 2];
    let mut cont = 0;
    let mut voto = 1;
    let mut promedio : [i32; 2] = [0 ; 2];
 
    loop {
        loop{
            arreglo_V[voto] =arreglo_V[voto] + candidatos [voto];
            arreglo_b[voto] = arreglo_b[voto] + candidatos[voto];
            println!("¿Han terminado todas las personas de votar a este candidato?, si es asi ingrese el numero 1 si no ingrese el numero 2");
            let mut entrada : String = String::new();
            stdin () . read_line (& mut entrada ) . unwrap () ;
            let numero : u32 = entrada
                . trim ()
                . parse ()
                . unwrap () ;  
            if numero == 3  {panic!("
                A ingresado un valor incorrecto"); }
            println!("{:?}", arreglo_V);
            if numero == 1{
                break
            }
            }
        cont += 1;
        voto += 1; 
        if cont == 25{
            break
        }
    }
    cont = 0;
    loop{
        cont += 1;
        suma[1] = suma[1] + arreglo_V[cont];
        if cont == 25{
            println!("la suma total de votos es {:?}", suma);
            break
        }
    }
    promedio[1] = suma[1] / 25;
    println!("El promedio de votos es  {:?}",promedio);
    cont = 0;
   
    
}

