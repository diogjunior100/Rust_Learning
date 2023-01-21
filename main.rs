fn main(){
    let variavel = 128;

    println!("Numero {}, tamanho {} bytes", variavel, std::mem::size_of_val(&variavel));
}