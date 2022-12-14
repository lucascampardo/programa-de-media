fn main() {
    let nota1: u8 = 10;
    let nota2: u8 = 6;
    let nota3: u8 = 9;
    let nota4: u8 = 4;
    let media: u8 =  (nota1 + nota2 + nota3 + nota4) / 4;

    if media >= 7 {
        println!("Your media is {media}, which is greater than 7");
    } else if media < 7 {
        println!("Your media is {media}, which is less than 7");
    } 
}