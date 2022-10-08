use rand::Rng;
use std::fmt;
use std::io::stdin;
fn main(){
    let mut arreglo : [i32 ; 14] = [0 ; 14];
    let mut arreglo2: [i32 ; 14] = [0 ; 14];
    let mut arreglo_c: [i32 ; 2] = [0 ; 2];
    let mut cont = 0;
    let mut V : String = String::new();
    loop{
         let mut numeros = g_numero();
        cont += 1;
        arreglo[cont] = numeros;
        arreglo2[cont] = numeros;
        if cont == 12 {
            break
        }
    }
    for i in 0..14{
        V = format!("{} {}", V, arreglo2[i].to_string())
    }
   

    arreglo.sort();
    println!("{:?}", arreglo);
    let mut vida = 500;
    loop{
        println!("¿cuales son los numeros?");
        let mut entrada : String = String::new();
        stdin () . read_line (& mut entrada ) . unwrap () ;
        let numero : i32 = entrada
            . trim ()
            . parse ()
            . unwrap ();

        for i in 0..14 {
            if arreglo[i] != numero{
                vida = vida - 1 ;
                if arreglo[i] == numero{
                    vida = vida;
                    println!("su total de vida es {}", vida );
                }
                println!("su total de vida es {}", vida );
                if vida == 0{
                    println!("usted a perdido");
                    break
                }
            

            }
        }
        

    }
    
   
}
fn g_numero()->i32 {
    let mut rng = rand::thread_rng();
    let  numero : i32 = rng.gen_range(0..100000);
    println!("{}", numero);
    return numero ;
}
