
static mut GLOBAL:u8 = 1; //forma de se declara uma variável global e mutável
                          //variáveis globais mutáveis são perigosas, pois podem ser alteradas por qualquer função
                          //obrigatóriamente tipada 
                          //o rust indica que seja escrita em letras maiúsculas      

fn main(){

  unsafe{ //definir que não é segura e que vocÊ sabe o que está fazendo
    println!("value = {}, tamanho = {} bytes", GLOBAL, std::mem::size_of_val(&GLOBAL));
  }

  const pi:f32 = 3.14; //obrigatóriamente tipada quando for constante 
  println!("value = {}, tamanho = {} bytes", pi, std::mem::size_of_val(&pi));

  let variable:u8 = 128;
  println!("value = {}, tamanho = {} bytes", variable, std::mem::size_of_val(&variable));

  let decimal:f32 = 2.5;
  println!("value = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

  let mut boolean:bool = false;
  boolean = true;
  println!("value = {}, tamanho = {} bytes", boolean, std::mem::size_of_val(&boolean));

  let letra:char = 'C';
  println!("value = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));

}