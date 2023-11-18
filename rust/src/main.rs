// Importamos las funciones y estructuras necesarias desde la crate lambda_runtime
// y también serde_json para el manejo de datos JSON.
use lambda_runtime::{service_fn, LambdaEvent, Context, Error};
use serde_json::{Value, json};

// Definimos una función asíncrona para manejar eventos de Lambda.
// `LambdaEvent<Value>` encapsula tanto el evento entrante como el contexto de ejecución.
async fn hello_world(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // `event.context` proporciona información contextual sobre la invocación de Lambda,
    // como el ID de la ejecución, ARN de la función, etc.
    let _context: Context = event.context;

    // Aquí manejarías el evento. Por ahora, simplemente respondemos con un JSON.
    // `json!` es una macro de serde_json que crea objetos JSON de manera conveniente.
    Ok(json!({ "message": "Hola Mundo desde AWS Lambda, Rust y Terraform. #IaC... :)" }))
}

// La función principal del programa.
// `#[tokio::main]` transforma la función en una rutina asíncrona gestionada por Tokio.
#[tokio::main]
async fn main() -> Result<(), Error> {
    // `service_fn` convierte nuestra función asíncrona `hello_world` en un servicio compatible con Lambda.
    // Esto es necesario porque Lambda espera un tipo específico que cumpla con la interfaz de servicio.
    let func = service_fn(hello_world);

    // `lambda_runtime::run` inicia el runtime de AWS Lambda y comienza a escuchar eventos.
    // Cuando un evento llega, invoca `func`, nuestra función `hello_world`.
    lambda_runtime::run(func).await
}
