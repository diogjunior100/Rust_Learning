const PI:f32 = 3.14;
static GLOBAL:u32 = 1;

fn main(){

    //variaveis diferentes tipos
    let variavel:i32 = 128;
    let decimal: f32 = 2.5;
    let mut boolean:bool = false;
    boolean = true;
    let letra:char = 'a';

    println!("Inteiro {}, tamanho {} bytes", variavel, std::mem::size_of_val(&variavel));
    println!("Decimal {}, tamanho {} bytes", decimal, std::mem::size_of_val(&decimal));
    println!("Boolean {}, tamanho {} bytes", boolean, std::mem::size_of_val(&boolean));
    println!("Letra {}, tamanho {} bytes", letra, std::mem::size_of_val(&letra));

    //constantes
    println!("Constantes {}", PI);
    println!("Global {}", GLOBAL);

}