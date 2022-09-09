use std::io::stdin;
fn main(){
    loop{
        let mut time = 0;
        println!("ingrese su codigo ");
        let mut isbn: String = String::new();
        let mut clean_isbn: String = String::new();
        stdin().read_line(&mut isbn).unwrap();

        for c in isbn.to_string().trim().chars(){
            if is_isbn_format_valid(&c.to_string()){
                clean_isbn = clean_isbn + &c.to_string(); 
            }
        }

        println!("{}", clean_isbn);
        if is_isbn10(clean_isbn){
            println!("{} es un ISBN10 valido", clean_isbn);
            time = time + 1 
        } else {
            println!("Su codigo no es valido, intente nuevamente ");
            let mut isbn: String = String::new();
            let mut clean_isbn: String = String::new();
            stdin().read_line(&mut isbn).unwrap();
            continue;
        }
        if time == 1{
        break;
        }
    }

}
fn is_isbn_format_valid(c: &str) -> bool {

    
    if c.chars().next().unwrap().is_numeric() {
        return true;
    } else if c == "X" || c == "x"{
        return true;
    }
    return false
}
fn is_isbn10() -> bool{
    clean_isbn
    

    

    

    return true
}


