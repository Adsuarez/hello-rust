fn main() {
    mutable_and_ummutable();
    constant();
    shadowing();
    tuples();
    arrays();
    cycles();
    example_factorial();
    example_prime_number();
    example_string_slice();
    copy_and_move();
    concatenate_str();
    strings();
    ownership_in_parameters();
    ownership_in_parameters_strings();
    exercise_1_ownership();
    struct_as_tuple();
    struct_without_implementation();
    struct_update_syntax();
    struct_with_implementation();
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
    title("3. SHADOWING");
    let a: i32 = 1;
    note("Se crea la primera vez la variable 'let a = 1'");
    let a: i32 = a + 23;
    note(
        "se vuelve a crear la variable 'let a = a + 23' lo que primero recupera el valor previo de 'a' y luego lo asigna a la nueva variable 'let a'"
    );
    let a: i32 = a - 23 / 23;
    note("se vuelve a crear la variable como se explicó en la nota anterior");
    println!("El resultado de la variable 'a' es: {}", a);
    note(
        "El resultado es 1 debido a que la jerarquía de operaciones hace que primero se divida 23/23"
    );
}

fn tuples() {
    title("4. TUPLAS");
    let user: (char, bool, i8, f32) = ('A', true, 30, 3.14);
    println!("Los datos del usuario 'user' son:");
    println!("inicial: {}", user.0);
    println!("es hombre: {}", user.1);
    println!("edad: {}", user.2);
    println!("número favorito: {}", user.3);
}

fn arrays() {
    title("5. MATRICES O ARREGLOS");
    let numeros = [1, 2, 3, 4];
    println!("El primer elemento del array es: {}", numeros[0]);
    note(
        "si se intenta acceder a un valor por fuera del tamaño del array se obtiene un 'PANIC'. Por ejemplo: println!('{}',numeros[4]);"
    );
    subtitle1("5.1 ciclo for para iterar el array:");
    for i in numeros.iter() {
        println!("{}", i);
    }
}

fn cycles() {
    title("6. CICLOS WHILE Y LOOP");
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
    note("el loop se interrumpe al escribir la palabra 'break'");
}

fn example_factorial() {
    let numero_factorial: i128 = 3;
    let resultado_factorial = factorial_calculation(numero_factorial);
    println!(
        "el resultado del factorial para el valor {} es: {}",
        numero_factorial,
        resultado_factorial
    )
}

fn factorial_calculation(numero: i128) -> i128 {
    title("7. CONDICIONALES, EJEMPLO 1: ");
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

fn example_prime_number() {
    let numero_incognita: u128 = 999983;
    let resultado_es_primo = is_prime(numero_incognita);
    println!("¿el número {} es primo? = {}", numero_incognita, resultado_es_primo);
}

fn is_prime(numero: u128) -> bool {
    title("8. CONDICIONALES, EJEMPLO 2: ");
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

fn example_string_slice() {
    note(
        "desde la función 'example_string_slice' se intenta crear una variable que reciba el resultado de la función 'string_slice', pero esto genera un error. Ejemplo: let recibe_str = string_slice();"
    );
    string_slice();
}

fn string_slice() {
    title("8. STR PEDAZO O PIEZA DE STRING");
    let saludo = "hola";
    println!("{}", saludo);
    println!("{}", saludo);
    note(
        "Si al finalizar una función intentamos retornar un 'str' genera un error, ya que 'str' no contiene el razgo Sized por lo que en tiempo de ejecución no se puede conocer el tamaño de dicho valor"
    );
    note(
        "Después de crear la variable de tipo 'str' y se haya usado, dicha variable es eliminada del stack de memoria por RUST, ya que no se vuelve a usar en el código, y de esta manera RUST asegura la limpieza de la memoria"
    );
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

fn concatenate_str() {
    title("10. CONCATENATE STR");
    let mut texto_1 = "palabra";
    println!("{}", texto_1);
    texto_1 = "nueva palabra";
    println!("{}", texto_1);
    let mut texto_2 = "añadida";
    println!("{}", texto_2);
    note(
        "si se intenta 'let concatenar = concat!(texto_1, texto_2);' Esto arroja error dado a que espera solo literales como 'hola', más no un str"
    );
    note(
        "si se intenta 'let mut concatenar = texto_1 + texto_2;' esto arroja error dado a que no se pueden sumar str porque requiere que la adisión tenga un Ownership"
    );
    texto_2 = "fue añadida";
    println!("{} {}", texto_1, texto_2);
    note(
        "usar la función 'println!('{} {}', texto_1, texto_2);' sería la única forma de unir dos str"
    );
    note(
        "en conclusión como el 'str' se almacena en STACK, debe tener un tamaño fijo conocido. Y al definir la variable con 'mut', lo único que puedo hacer es reasignarle un valor, pero no mutar el valor en sí mismo."
    );
}

fn strings() {
    title("11. STRINGS");
    let texto_1 = String::from("Texto inmutable");
    println!("la variable texto_1 es inmutable y su contenido es: {}", texto_1);
    let mut texto_2 = String::from("texto mutable");
    println!("la variable texto_2 es mutable y su contenido inicial es: {}", texto_2);
    texto_2 = texto_2 + " gracias a la adición con el símbolo '+'";
    println!("la variable texto_2 ha sido mutada y su nuevo contenido es: {}", texto_2);
}

fn ownership_in_parameters() {
    title("12. PROPIEDAD (OWNERSHIP) AL PASAR PARÁMETROS A UNA FUNCIÓN");
    subtitle1("12.1 con un número que hace el copy automático");
    let numero = 20;
    fn imprimir_numero(parametro: i32) {
        println!("desde la función 'imprimir_numero' el valor del parámetro es: {}", parametro)
    }
    imprimir_numero(numero);
    println!("el valor de la variable 'numero' después de pasarse como parámetro es: {}", numero);
    note(
        "Al poder imprimir la variable número luego de haberla pasado como parámetro, se demuestra que numero no ha dejado de existir, lo que indica que solo se copió"
    );
    subtitle1("12.2 con 'str' que no se puede copiar, solo pasar como referencia");
    let pieza_de_string = "TEXTO";
    fn imprimir_pieza_de_texto(slice: &str) {
        note(
            "si aquí no se pone el símbolo '&' genera un error, ya que se está obligado a pasar la referencia del 'str' y no su valor"
        );
        println!("desde la función 'imprimir_pieza_de_texto' el valor del parámetro es: {}", slice)
    }
    imprimir_pieza_de_texto(&pieza_de_string);
    println!("el valor de la variable 'pieza_de_string' después de pasarse como parámetro es: {}", pieza_de_string)
}

fn ownership_in_parameters_strings() {
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

fn struct_as_tuple() {
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

fn struct_without_implementation() {
    title("15. STRUCT SIN IMPLEMENTACIÓN");
    #[derive(Debug)]
    struct Suscriptor {
        es_inteligente: bool,
        nombre: String,
        seguidores: u128,
    }
    note(
        "El trait '#[derive(Debug)]' es obligatorio escribirlo antes de la definición de la estructura, cuando se vaya a imprimir la estructura completa sin acceder a sus campos, o generará error"
    );
    subtitle1(
        "15.1. Opción 1: el parámetro de la función tiene un nombre distinto del campo de la estructura"
    );
    fn nuevo_suscriptor_opcion1(nueva_persona: String) -> Suscriptor {
        Suscriptor {
            es_inteligente: true,
            nombre: nueva_persona,
            seguidores: 1550,
        }
    }
    subtitle1(
        "15.2. Opción 2: el parámetro de la función tiene el mismo nombre del campo de la estructura"
    );
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
    note(
        "La impresión de la estructura completa de 'suscriptor1' y 'suscriptor2' fue posible con el trait '#[derive(Debug)]'"
    );
    note(
        "Si no se accede a alguno de los campos de la estructura, se genera un warning, por lo cual se imprimen los campos a continuación para dejar limpio el output"
    );
    println!("Los datos separados del suscriptor 1 son:");
    println!("¿es inteligente?: {}", suscriptor1.es_inteligente);
    println!("nombre: {}", suscriptor1.nombre);
    println!("Cantidad de seguidores: {}", suscriptor1.seguidores)
}

fn struct_update_syntax() {
    title("16. STRUCT UPDATE SYNTAX (PROPAGACIÓN/SPREAD)");
    struct House {
        m2: u128,
        rooms: u8,
    }
    let house1: House = House { m2: 1250, rooms: 2 };

    println!(
        "La casa 'house1' tiene {} metros cuadrados y {} habitaciones",
        house1.m2,
        house1.rooms
    );

    let house2: House = House { rooms: 3, ..house1 };
    println!(
        "La casa 'house2' tiene {} metros cuadrados y {} habitaciones",
        house2.m2,
        house2.rooms
    );
    note(
        "Al usar la sintaxis de actualización de struct '..struct' se propagan los campos reutilizables de la estructura"
    );
}

fn struct_with_implementation() {
    title("17. STRUCT CON IMPLEMENTACIÓN");
    struct Student {
        id: u32,
        age: u8,
        name: String,
        course: String,
        active: bool,
    }

    impl Student {
        //Función asociada (Constructor)
        fn new(id: u32, age: u8, name: String, course: String) -> Student {
            Student {
                id,
                age,
                name,
                course,
                active: true,
            }
        }
        fn leaves(&mut self) {
            self.active = false;
        }
    }

    let mut student1 = Student::new(123, 32, String::from("Pedro"), String::from("ingles"));

    println!("los datos del nuevo estudiante 'student1' son: ");
    println!("id = {}", student1.id);
    println!("edad = {}", student1.age);
    println!("nombre = {}", student1.name);
    println!("curso = {}", student1.course);
    println!("activo = {}", student1.active);

    student1.leaves();
    println!(
        "Después de ejecutar la función asociada 'leaves()' para el estudiante 'student1' sus datos son: "
    );
    println!("id = {}", student1.id);
    println!("edad = {}", student1.age);
    println!("nombre = {}", student1.name);
    println!("curso = {}", student1.course);
    println!("activo = {}", student1.active);
}
