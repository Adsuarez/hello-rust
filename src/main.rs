fn main() {
    mutable_and_ummutable();
    constant();
    shadowing();
    tuples();
    arrays();
    ciclos();
    ejemplo_factorial();
    ejemplo_primo();
    ejemplo_string_slice();
    copy_and_move();
    prueba_concatenar_str();
    strings();
    ownership_en_parametros();
    ownership_en_parametros_strings();
    exercise_1_ownership();
    estructura_como_tupla();
    estructura_sin_implementacion();
}

fn title(texto: &str) {
    println!("\n{}", texto);
}

fn subtitle1(texto: &str) {
    println!("  {}", texto)
}

fn note(text: &str) {
    println!(" -> NOTE: {}", text)
}

fn mutable_and_ummutable() {
    title("1. MUTABLE E INMUTABLE");
    let x = 2;
    println!("Se crea la variable 'let x' y su valor es: x = {}", x);
    note(
        "Al intentar reasignar 'x = 3' se genera un error ya que Por defecto 'let' crea variables inmutables"
    );
    let mut y = 5;
    println!("el valor de la variable mutable 'y' es: {}", y);
    y = 8;
    println!("el nuevo valor de la variable mutable 'y' es: {}", y);
    note(
        "al agregar 'mut' se le indica al programa que el valor de la misma podrá ser cambiado o reasignado. En el ejemplo inicia 'y = 5' y luego cambia a 'y = 8'"
    );
}

fn constant() {
    title("2. CONSTANTE");
    const PI: f64 = 3.141516;
    println!("el valor de la constante 'PI' es: {}", PI);
    note("Se recomienda que las constantes sean 'upper case snake case' ejemplo: EDAD_MINIMA");
}

fn shadowing() {
    title("3. PRUEBA SHADOWING");
    let a: i32 = 1;
    let a: i32 = a + 23;
    let a: i32 = a - 23 / 23; //el resultado es 1 debido a que la jerarquía de operaciones hace que primero se divida 23/23
    println!("El resultado de la variable 'a' es: {}", a);
}

fn tuples() {
    title("4. Tuplas");
    let user: (char, bool, i8, f32) = ('A', true, 30, 3.14);
    println!("Los datos del usuario 'user' son:");
    println!("inicial: {}", user.0);
    println!("es hombre: {}", user.1);
    println!("edad: {}", user.2);
    println!("número favorito: {}", user.3);
}

fn arrays() {
    title("5. Matrices");
    let numeros = [1, 2, 3, 4];
    println!("El primer elemento del array es: {}", numeros[0]);
    //println!("si me desbordo de la longitud del array obtengo: {}",numeros[4]); //error: "this operation will panic at runtime"
    println!("5.1 ciclo for para iterar el array:");
    for i in numeros.iter() {
        println!("{}", i);
    }
}

fn ciclos() {
    title("6. ciclos While y Loop");
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
    println!("al final de loop el valor de la variable 'numero' es: {}", numero);
}

fn ejemplo_factorial() {
    let numero_factorial: i128 = 3;
    let resultado_factorial = calcular_factorial(numero_factorial);
    println!(
        "el resultado del factorial para el valor {} es: {}",
        numero_factorial,
        resultado_factorial
    )
}

fn calcular_factorial(numero: i128) -> i128 {
    title("7. Ejemplo 1: Condicionales");
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
    println!("¿el número {} es primo? = {}", numero_incognita, resultado_es_primo);
}

fn es_primo(numero: u128) -> bool {
    title("8. Ejemplo 2: Condicionales");
    let mut resultado = false;
    let numero_normalizado = numero as f64;
    if numero <= 1 {
        return resultado;
    }
    for i in 2..(numero_normalizado.sqrt() as u128) + 1 {
        if numero % i == 0 {
            return resultado;
        }
    }
    resultado = true;
    return resultado;
}

fn ejemplo_string_slice() {
    //let recibe_str = string_slice(); //->VER NOTA A
    string_slice();
}

fn string_slice() {
    title("8. str (pedazo de string)");
    let saludo = "hola";
    println!("{}", saludo);
    println!("{}", saludo);
    //return saludo //->NOTA A: si intentamos retornar la variable, nos genera un error ya que 'str' no contiene el razgo Sized por lo que en tiempo de ejecución nos e puede conocer el tamaño de dicha variable
    //Después de que se imprima la variable saludo, dicha variable es eliminada del stack de memoria por RUST, ya que no se vuelve a usar en el código
}

fn copy_and_move() {
    title("9. Copy, Move y Clone");
    //copy
    let var1 = 2;
    let var2 = var1;
    let mut var3 = var1 + var2;
    var3 = var3 + 1;
    println!("el valor de var1 es: {}, de var2 es: {} y de var3 es: {}", var1, var2, var3); //Aquí se comprueba que al asignarle una variable a otra variable, lo que realiza es una copia del valor

    //move
    let s1 = String::from("texto1");
    let s2 = s1;
    //println!("el contenido de s1 es: {} y de s2 es: {}", s1, s2);//->Aquí se comprueba que los strings al ser almacenados en el HEAP y no en el STACK, RUST automáticamente mueve el puntero de una variable a la otra al momento de asignarlas. Por lo que esta línea genera un error ya que s1 deja de existir
    println!("el valor de s2 es: {}", s2);

    //copy de strings (clone)
    let texto_1 = String::from("palabra");
    let texto_2: String = texto_1.clone();
    println!("la variable texto_1 es: {}", texto_1);
    println!("la variable texto_2 es: {}", texto_2);
}

fn prueba_concatenar_str() {
    title("10. prueba concatenar str");
    let mut texto_1 = "palabra";
    println!("{}", texto_1);
    texto_1 = "nueva palabra";
    println!("{}", texto_1);
    let mut texto_2 = "añadida";
    println!("{}", texto_2);
    //let concatenar = concat!(texto_1, texto_2); //->Esto arroja error dado a que espera solo literales como "hola", más no un str
    //let mut concatenar = texto_1 + texto_2;// -> esto arroja error dado a que no se pueden sumar str porque requiere que la adisión tenga un Ownership
    texto_2 = "fue añadida";
    println!("{} {}", texto_1, texto_2); //Esta sería la única forma de unir dos str
    //en conclusión como el str se almacena en STACK, debe tener un tamaño fijo conocido. Y al permitirme ponerle mut a la variable, lo único que puedo hacer es reasignarle un valor, pero no mutar el valor en sí mismo.
}

fn strings() {
    title("11. strings");
    let texto_1 = String::from("Texto inmutable");
    println!("la variable texto_1 es inmutable y su contenido es: {}", texto_1);
    let mut texto_2 = String::from("texto mutable");
    println!("la variable texto_2 es mutable y su contenido inicial es: {}", texto_2);
    texto_2 = texto_2 + " gracias a la adición con el símbolo '+'";
    println!("la variable texto_2 ha sido mutada y su nuevo contenido es: {}", texto_2);
}

fn ownership_en_parametros() {
    title("12. Propiedad al pasar parámetros a una función");
    //con valores que hacen el copy automático
    let numero = 20;
    fn imprimir_numero(parametro: i32) {
        println!("desde la función 'imprimir_numero' el valor del parámetro es: {}", parametro)
    }
    imprimir_numero(numero);
    println!("el valor de la variable 'numero' después de pasarse como parámetro es: {}", numero); //Esto demuestra que numero no ha dejado de existir aunque su valor se pasó como parámetro, lo que indica que solo se copió
    //con str que no se puede copiar, solo pasar como referencia
    let pieza_de_string = "TEXTO";
    fn imprimir_pieza_de_texto(slice: &str) {
        //aquí si no se pone el símbolo '&' genera un error, ya que se está obligado a pasar la referencia del str y no su valor
        println!("desde la función 'imprimir_pieza_de_texto' el valor del parámetro es: {}", slice)
    }
    imprimir_pieza_de_texto(&pieza_de_string);
    println!("el valor de la variable 'pieza_de_string' después de pasarse como parámetro es: {}", pieza_de_string)
}

fn ownership_en_parametros_strings() {
    title("13. Propiedad al pasar Strings como parámetros a una función");
    //perder propiedad
    let texto = String::from("texto");
    fn prestar(parametro: String) -> String {
        return parametro;
    }
    let recibidor = prestar(texto); //la variable 'texto' deja de existir al pasarse como parámetro en la función 'prestar'
    println!("la variable 'recibidor' contiene el valor: {}", recibidor);

    //prestar propiedad
    let palabra = String::from("palabra");
    fn calcular_longitud_string(cadena: &String) -> usize {
        cadena.len()
    }
    let longitud = calcular_longitud_string(&palabra);
    println!(
        "La variable 'palabra' contiene el valor: {}, el cual tiene {} caracteres",
        palabra,
        longitud
    ); //Aquí se puede volver a usar la variable 'palabra' porque se prestó momentaneamente la referencia a su valor. Y al finalizar la función 'calcular_longitud_string' le devolvió la propiedad a la variable 'palabra'

    //prestar propiedad y modificar el valor
    let mut nombre = String::from("Janito");
    println!("Aquí la variable 'nombre' contiene el valor: {}", nombre);
    fn mutar_string(texto: &mut String) {
        texto.push_str(" alimaña");
    }
    mutar_string(&mut nombre);
    println!("Al pasar la variable 'nombre' a la función 'mutar_string' su valor es: {}", nombre);
}

fn exercise_1_ownership() {
    title("Exercise 1: Ownership");
    println!("-> Por favor introduce tu nombre: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    fn elimina_caracteres_del_enter(text: &mut String) {
        text.pop();
        text.pop();
    }
    elimina_caracteres_del_enter(&mut name);
    fn add_string(text: &mut String) {
        text.insert_str(0, "   Chao ");
    }
    fn longitud_string(text: &String) -> usize {
        text.len()
    }
    let longitud = longitud_string(&name);
    println!("   Tu nombre {} está formado por {} caracteres", name, longitud);
    add_string(&mut name);
    println!("{}", name);
}

fn estructura_como_tupla() {
    title("14. struct como tupla");
    struct RGBColor(u8, u8, u8);
    let rojo = RGBColor(255, 0, 0);
    let celeste = RGBColor(0, 255, 255);
    println!("el código RGB para el color rojo es: ({}, {}, {})", rojo.0, rojo.1, rojo.2);
    println!(
        "el código RGB para el color celeste es: ({}, {}, {})",
        celeste.0,
        celeste.1,
        celeste.2
    )
}

fn estructura_sin_implementacion() {
    title("15. struct sin implementación");
    #[derive(Debug)] //este trair es obligatorio escribirlo al principio de la función que intente imprimir la struct completa, o generará error
    struct Suscriptor {
        es_inteligente: bool,
        nombre: String,
        seguidores: u128,
    }
    //Opción 1: el parámetro de la función tiene un nombre distinto del campo de la estructura
    fn nuevo_suscriptor_opcion1(nueva_persona: String) -> Suscriptor {
        Suscriptor {
            es_inteligente: true,
            nombre: nueva_persona,
            seguidores: 1550,
        }
    }
    //Opción 2: el parámetro de la función tiene el mismo nombre del campo de la estructura
    fn nuevo_suscriptor_opcion2(nombre: String) -> Suscriptor {
        Suscriptor {
            es_inteligente: false,
            nombre,
            seguidores: 19,
        }
    }
    let suscriptor1 = nuevo_suscriptor_opcion1(String::from("Pedro"));
    let suscriptor2 = nuevo_suscriptor_opcion2(String::from("Ruben"));
    println!("Los datos del suscriptor 1 son: {:?}", suscriptor1);
    println!("Los datos del suscriptor 2 son: {:?}", suscriptor2);
    println!("Los datos separados del suscriptor 1 son:");
    //las siguientes 3 lineas se debieron implementar para eliminar el warning que aparece al no haber accedido a los campos de la estructura en las impresiones de suscriptor1 y suscriptor2
    println!("¿es inteligente?: {}", suscriptor1.es_inteligente);
    println!("nombre: {}", suscriptor1.nombre);
    println!("Cantidad de seguidores: {}", suscriptor1.seguidores)
}
