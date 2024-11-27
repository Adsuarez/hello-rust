fn main() {
    prueba_inmutable();
    prueba_constante();
    shadowing();
    tuples();
    arrays();
    ciclos();
    ejemplo_factorial();
    ejemplo_primo();
}

fn inicio_funcion(texto: &str) {
    println!("");
    println!("{}", texto);
}

fn prueba_inmutable() {
    inicio_funcion("1. PRUEBA INMUTABLE");
    let x = 2;
    println!("el valor de la variable inmutable 'x' es: {}", x);
    //x = 2; //Por defecto let crea variables inmutables
    let mut y = 5;
    println!("el valor de la variable mutable 'y' es: {}", y);
    y = 8;
    println!("el nuevo valor de la variable mutable 'y' es: {}", y);
}

fn prueba_constante() {
    inicio_funcion("2. PRUEBA CONSTANTE");
    const PI: f64 = 3.141516;
    println!("el valor de la constante 'PI' es: {}", PI);
}

fn shadowing() {
    inicio_funcion("3. PRUEBA SHADOWING");
    let a: i32 = 1;
    let a: i32 = a + 23;
    let a: i32 = a - 23 / 23; //el resultado es 1 debido a que la jerarquía de operaciones hace que primero se divida 23/23
    println!("El resultado de la variable 'a' es: {}", a);
}

fn tuples() {
    inicio_funcion("4. Tuplas");
    let user: (char, bool, i8, f32) = ('A', true, 30, 3.14);
    println!("Los datos del usuario 'user' son:");
    println!("inicial: {}", user.0);
    println!("es hombre: {}", user.1);
    println!("edad: {}", user.2);
    println!("número favorito: {}", user.3);
}

fn arrays() {
    inicio_funcion("5. Matrices");
    let numeros = [1, 2, 3, 4];
    println!("El primer elemento del array es: {}", numeros[0]);
    //println!("si me desbordo de la longitud del array obtengo: {}",numeros[4]); //error: "this operation will panic at runtime"
    println!("5.1 ciclo for para iterar el array:");
    for i in numeros.iter() {
        println!("{}", i);
    }
}

fn ciclos() {
    inicio_funcion("6. ciclos While y Loop");
    let mut numero: i32 = 0;
    println!("la variable 'numero' inicia en '0'");
    while numero < 10 {
        numero = numero + 1;
        println!("la variable 'numero' ahora es: {}", numero);
    }
    loop {
        numero = numero - 10;
        break;
    }
    println!(
        "al final de loop el valor de la variable 'numero' es: {}",
        numero
    );
}

fn ejemplo_factorial() {
    let numero_factorial: i128 = 3;
    let resultado_factorial = calcular_factorial(numero_factorial);
    println!(
        "el resultado del factorial para el valor {} es: {}",
        numero_factorial, resultado_factorial
    )
}

fn calcular_factorial(numero: i128) -> i128 {
    inicio_funcion("7. Ejemplo 1: Condicionales");
    if numero == 0 || numero == 1 {
        1
    } else {
        let mut result = numero;
        for i in (1..numero).rev() {
            result = result + i;
        }
        result
    }
}

fn ejemplo_primo() {
    let numero_incognita: u128 = 999983;
    let resultado_es_primo = es_primo(numero_incognita);
    println!(
        "¿el número {} es primo? = {}",
        numero_incognita, resultado_es_primo
    );
}

fn es_primo(numero: u128) -> bool {
    inicio_funcion("8. Ejemplo 2: Condicionales");
    let mut resultado = false;
    let numero_normalizado = numero as f64;
    if numero <= 1 {
        return resultado;
    }
    for i in 2..((numero_normalizado.sqrt() as u128) + 1) {
        if numero % i == 0 {
            return resultado;
        }
    }
    resultado = true;
    return resultado;
}
