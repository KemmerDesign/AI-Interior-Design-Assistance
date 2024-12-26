use actix_web::{web, HttpResponse, Responder, http::header::ContentType};
use crate::models::data_request::UserInput;
use super::super::services::image_service;
use crate::services::ai_service;

pub async fn generate(input: web::Json<UserInput>) -> impl Responder {
    println!("Datos recibidos: {:?}", input);//Para controlar que la información se este recibiendo correctamente

    // Decodificar la imagen base64
    // El usar match es para manejar los errores que se puedan presentar en la decodificación de la imagen
    // esto nos evita tener que usar estructura de flujo como if else y nos permite manejar los errores de una manera más limpia

    let image = match image_service::decode_base64_to_image(&input.image) {
        Ok(image) => image,
        Err(e) => {//acá se atrapa el error y se imprime en consola
            eprintln!("Error al decodificar la imagen base64: {:?}", e);
            return HttpResponse::BadRequest().body("Error al decodificar la imagen");
        }
    };

    // Preprocesar la imagen

    let preprocessed_image = match image_service::preprocess_image(&image) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Error al preprocesar la imagen: {:?}", e);
            return HttpResponse::InternalServerError().body("Error al preprocesar la imagen");
        }
    };

    // Codificar la imagen preprocesada a bytes
    let preprocessed_image_bytes = match image_service::encode_image(&preprocessed_image) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error al codificar la imagen preprocesada: {:?}", e);
            return HttpResponse::InternalServerError().body("Error al codificar la imagen");
        }
    };

    // Construir el prompt para la API de HuggingFace
    let prompt = format!("Generate an interior design image in a {} style, incorporating {} furniture and {} walls, based on the uploaded image. Ensure the design reflects a cohesive and aesthetically pleasing arrangement, suitable for a modern home setting.", input.style, input.furniture, input.color);

    // Enviar la imagen y el prompt a la API de HuggingFace
    let generated_image_bytes = match ai_service::generate_image(preprocessed_image_bytes, prompt).await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error al generar la imagen con HuggingFace: {:?}", e);
            return HttpResponse::InternalServerError().body("Error al generar la imagen");
        }
    };

    // Devolver la imagen generada
    HttpResponse::Ok()
        .content_type(ContentType::jpeg())
        .body(generated_image_bytes)
}