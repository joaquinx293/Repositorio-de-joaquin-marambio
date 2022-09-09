use std::io::stdin;
fn main(){
    println!("ingrese su cadena de ADN");// creo un input para pregunta
    let mut ADN: String = String::new();// genero una variable 
    stdin().read_line(&mut  ADN).unwrap();
    let kronos = 0 ;//kronos es un contador 
    loop {
        if ADN.contains(""){
            if ADN.contains("G"){// busca la letra 
                print!("C");//escribe en la terminal su complemento 
                let kronos = kronos + 1;// suma 1 al contador 
            }
            if ADN.contains("C"){
                print!("G");
                let kronos = }kronos + 1;
            }
            if ADN.contains("A"){
                print!("U");
                let kronos = kronos + 1;
            }
            if ADN.contains("T"){
                print!("A");
                let kronos = kronos + 1;
            }
        if _kronos == 4{
            println!("Su cadena fue convertida");
                break; // este if cerrara el loop
            }    
        }else{
            println!("ingrese una cadena valida de ADN");
            let mut ADN: String = String::new();
            stdin().read_line(&mut  ADN).unwrap();
            continue;
        }
        

    