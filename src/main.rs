use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Adivinhe um numero!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);
      
    loop {
      let mut numero = String::new();

      io::stdin()
          .read_line(&mut numero)
          .expect("Falha nessa linha");

     let numero:u32 = match numero.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("Por favor informe um numero");

      match numero.cmp(&numero_secreto){
          Ordering::Less => println!("Numero mais baixo"),
         Ordering::Greater => println!("Numero maior"),
         Ordering::Equal => {
           println!("Voce acertou!");
           break;
         }
      }

    }
  
}
