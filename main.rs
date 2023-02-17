fn main(){
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