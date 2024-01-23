# Rust para contratos inteligentes

# Ejemplos de Rust para Tutorial

A continuación, se presentan algunos ejemplos de código en Rust que destacan sus propiedades más importantes. 

## 1. Manejo de Memoria y Propiedad

### En este ejemplo, se muestra el manejo de la memoria y la transferencia de propiedad en Rust. La asignación en el heap y la transferencia de propiedad son fundamentales para garantizar la seguridad y evitar problemas de gestión de memoria.

```rust
fn main() {
    let data = Box::new(42); // Asignación en el heap
    let otra_data = data;     // Transferencia de propiedad
    // No se puede acceder a 'data' aquí porque la propiedad ha sido transferida
    // println!("data: {}", data); // Esto generaría un error
    println!("otra_data: {}", otra_data);

}

```


## 2. Concurrencia Sin Riesgos

### En este ejemplo, se ilustra la concurrencia sin riesgos en Rust utilizando hilos. La concurrencia sin riesgos permite la ejecución paralela sin preocuparse por problemas típicos como condiciones de carrera y bloqueos.


```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Hilo: {}", i);
        }
    });

    for i in 1..=3 {
        println!("Principal: {}", i);
    }

    // Espera a que el hilo termine antes de continuar
    handle.join().unwrap();
}


```


## 3. Sistema de Tipos Fuerte

### En este ejemplo, se destaca el sistema de tipos fuerte de Rust. El sistema de tipos ayuda a prevenir errores al verificar la corrección de tipos en tiempo de compilación, mejorando así la seguridad del código.

```rust

fn suma(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultado = suma(5, 7);
    // Intentar pasar una cadena como argumento generaría un error de compilación
    // let resultado_invalido = suma("Hola", "Rust"); // Esto generaría un error
    println!("Resultado: {}", resultado);
}

```


## 4. Patrones de Desestructuración y Coincidencia

### En este ejemplo, se muestra cómo usar patrones de desestructuración y coincidencia en Rust. Estas características permiten trabajar con estructuras de datos de manera concisa y expresiva.

```rust

struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let punto = Punto { x: 10, y: 20 };

    // Desestructuración
    let Punto { x, y } = punto;

    println!("Coordenadas: x = {}, y = {}", x, y);
}


```
## 5. Cierre (Closure) y Funciones de Orden Superior

### En este ejemplo, se explora el uso de cierres (closures) y funciones de orden superior en Rust. Estas características permiten escribir código más genérico y flexible, facilitando la implementación de operaciones sobre funciones.

```rust

fn operacion_binaria(a: i32, b: i32, operacion: fn(i32, i32) -> i32) -> i32 {
    operacion(a, b)
}

fn suma(a: i32, b: i32) -> i32 {
    a + b
}

fn resta(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let resultado_suma = operacion_binaria(5, 3, suma);
    let resultado_resta = operacion_binaria(8, 4, resta);

    println!("Suma: {}", resultado_suma);
    println!("Resta: {}", resultado_resta);
}




```


