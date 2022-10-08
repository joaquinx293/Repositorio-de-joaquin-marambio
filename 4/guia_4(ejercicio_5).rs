use rand::Rng;
fn main(){
    let mut matriz : [[ i32 ;50];50] = [[1;50];50];
    println!("{:?}", matriz);
    let mut numero_1 = g_numero();
    let mut cont  = 0;
    let mut rng = rand::thread_rng();
    let  numero : i32 = rng.gen_range(1..numero_1);
    let mut numero_2 = numero;
    let mut matriz_2 : [[ i32 ;numero_2];numero_2 = [[1;numero_2];numero_2];
    println!("{:?}", matriz_2)
    loop{
        cont += 1;
        matriz[cont] = [numero_1 ; 50];
        if cont == 10{
            break
        }

    }
    
}
fn g_numero()->i32 {
    let mut rng = rand::thread_rng();
    let  numero : i32 = rng.gen_range(1..51);
    println!("{}", numero);
    return numero ;
}
