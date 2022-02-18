fn main() {
    // macros
    println!("Hello {}, en {}", "mundo", "rust");
    // variables = por defecto son inmutables
    let a_number: i32 = 22;
   // a_number = 23;  = no funciona porque no es inmutable
   // con let mut podemos indicar que será una variable mutable
   let mut b_number: i32 = 22;
    println!("el valor de a number es: {}", a_number);
    println!("el valor de b number es: {}", b_number);
    b_number = 23;
    println!("el valor de b number cambió a : {}", b_number);
}
