use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde_json::json;
use std::error::Error;
use std::env;

pub async fn generate_image(image_bytes: Vec<u8>, prompt: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let api_token = env::var("HUGGINGFACE_API_TOKEN") //Leemos en las variables de entorno la variable HUGGINGFACE_API_TOKEN
        .map_err(|_| "Error: HUGGINGFACE_API_TOKEN no está configurada")?;//encaso de no estar configurada se lanza un error

    //Aca construimos el cuerpo de la petición la cabecera y el cuerpo de la petición
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_token))?);
    headers.insert("Content-Type", HeaderValue::from_static("application/octet-stream"));
    headers.insert("X-Wait-For-Model", HeaderValue::from_static("true"));

    // Construir el cuerpo de la petición
    let body = image_bytes;

    // Crear un cliente HTTP
    let client = Client::new();

    // Realizar la petición POST a la API de HuggingFace
    let response = client.post("https://api-inference.huggingface.co/models/stabilityai/stable-diffusion-3.5-large") // Reemplaza con la URL del modelo que vas a usar
        .headers(headers)
        .body(body)
        .send()
        .await?;

    // Verificar el código de estado de la respuesta
    if response.status().is_success() {
        // Leer la respuesta como bytes
        let image_bytes = response.bytes().await?.to_vec();
        Ok(image_bytes)
    } else {
        Err(format!("Error en la API de HuggingFace: {:?}", response.status()).into())
    }

}