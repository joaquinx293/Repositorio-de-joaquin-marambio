use std::io::stdin;
fn main(){
    let mut contador = 0;
    let mut toneladas : [i32 ; 13] = [0; 13];
    let mut suma : [i32 ; 2] = [ 0 ; 2];
    let mut promedio : [i32 ; 2] = [0 ; 2];
    let mut n_mes = 0;
    loop{
        contador += 1;
        println!("¿cuantas toneladas hay en este mes?");
        toneladas[contador] = numero();
        if contador == 12{
            break
        }
    }
    contador = 0;
    loop{
        contador += 1;
        suma[1] = suma[1] + toneladas[contador];
        if contador == 12{
            println!("el total de toneladas cosechadas este año es {:?}",suma);
            break
        }
    }
    promedio[1] = suma[1] / 12;

    contador = 0;
    loop{
        contador += 1;
        if promedio[1] < toneladas[contador] {
            n_mes +=1;
            println!("la producion mayor que el promedio fue {:?} toneladas,", toneladas[contador]);
            println!("en el {} mes", contador);

        }
        if contador == 12{
            break
        }
        
    }
    println!("la cantidad de meses en los que se produjo mas es {}",n_mes);


}
fn numero()-> i32{
    let mut Pregunta : String = String::new();
    stdin().read_line(&mut  Pregunta).unwrap();
    let mut numero : i32 = Pregunta
    .trim()
    .parse()
    .unwrap();
    return numero
}
