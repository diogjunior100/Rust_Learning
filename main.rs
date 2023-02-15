const PI:f32 = 3.14;
static GLOBAL:u32 = 1;

fn soma(a:i32, b:i32) -> i32{
    let soma:i32 = a + b;

    println!("A soma de {} + {} eh igual a {}", a, b, soma);

    return soma;

}

fn sombra(){
    let a = 123;

    {
        let b = 456;
        let a = 777;
        println!("dentro, b {}", b);
        println!("dentro, a {}", a);

    }
    //println!("fora, b {}", b);
    println!("fora, a {}", a);
    

}


fn escopo(){
    //variaveis diferentes tipos
    let variavel:i32 = 128;
    let decimal: f32 = 2.5;
    //let boolean:bool = false;
    //boolean = true;
    let letra:char = 'a';

    println!("Inteiro {}, tamanho {} bytes", variavel, std::mem::size_of_val(&variavel));
    println!("Decimal {}, tamanho {} bytes", decimal, std::mem::size_of_val(&decimal));
    //println!("Boolean {}, tamanho {} bytes", boolean, std::mem::size_of_val(&boolean));
    println!("Letra {}, tamanho {} bytes", letra, std::mem::size_of_val(&letra));

    //constantes
    println!("Constantes {}", PI);
    println!("Global {}", GLOBAL);




}

fn main(){
    escopo();
    sombra();

    println!("Soma {}", soma(2,2));
    println!();
 
    let idade = 42;
    let responsavel = false;

    if idade < 18 || responsavel {
        println!("nao pode entrar");

    }
    else if idade == 42 {
        println!("tambem nao pode")
    }
    else{
        println!("Pode entrar");
    }
}