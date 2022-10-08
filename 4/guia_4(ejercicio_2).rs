use rand::Rng;
fn main(){
    LISTA1();
}
fn LISTA1(){
    let mut N : [i32 ; 15] = [0; 15];
    let mut cont = 0;
    let mut orden : [i32 ; 12] = [0 ; 12];
    let mut nueva_lista : [i32 ; 25] = [0 ; 25];
    loop{
        cont += 1;
        if cont < 11 {
            let mut n2 = g_numero();
            N[cont] = n2;

        }
        if cont == 10{
            break
        }
    }
    let mut M : [i32 ; 12] = [0; 12];
    let mut cont = 0;
    loop{
        cont += 1;
        if cont < 11 {
            let mut m2 = g_numero();
            M[cont] = m2;
        }
        if cont == 11{
            break
        }
    }
    let mut cont2 = 0;
    let mut suma_F : [i32 ; 15] = [0 ; 15];
    loop{
        cont2 += 1;
        suma_F[cont2] = M[cont2] + N[cont2] ;
        println!("la suma de los numeros de los arreglos es : {:?}", suma_F);
        if cont2 == 11{
            break
        }
    }
    suma_F.sort();
    println!("La suma ordenada es :{:?}",suma_F)

}



fn g_numero()->i32 {
    let mut rng = rand::thread_rng();
    let  n : i32 = rng.gen_range(0..100);
    println!("{}", n);
    return n ;
}
