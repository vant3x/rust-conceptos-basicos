fn main() {
    // macros
    println!("Hello {}, en {}", "mundo", "rust");
    // variables = por defecto son inmutables
    let a_number: i32 = 22;
   // a_number = 23;  = no funciona porque no es inmutable
   // con let mut podemos indicar que será una variable mutable
   let mut b_number: i32 = 22;
    println!("el valor de a number es: {    }", a_number);
    println!("el valor de b number es: {}", b_number);
    b_number = 23;
    println!("el valor de b number cambió a : {}", b_number);
    let shadow_num = 2;
    println!("el valor de shadow numb es : {}", shadow_num);
    let shadow_num = shadow_num * 2;
    println!("el valor de shadow numb es : {}", shadow_num);
    let boolean: bool = true;
    let title: &str = "string hola";
    let stringChar = 'a';
    let stringChar2: char = 'b';
    println!("{}", stringChar);
    println!(" el char2 es {}", stringChar2);

    // tuplas
    // las tuplas en rust son inmutables  y no se pueden agregar o eliminar elementos
    let tuple: (&str, i32, bool, f64) = ("alejo", 22, true, 4.5);
    println!(" la tupla es {}", tuple.3);

    // structs
   struct Persona {nombre: String, edad: u8, github: String, remote: bool}
   // tuple struct
   struct Hobbies(String, String, String);
   let persona: Persona = Persona {
       nombre: String::from("Alejandro"),
       edad: 22,
       github: String::from("Vant3x"),
       remote: true
   }

   println!("{}", persona.nombre);


}
