use crate::services::ai_service;
use crate::services::image_service;
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{http::header::ContentType, Error, HttpResponse};
use std::io::Read;

#[derive(MultipartForm)]
pub struct ImageUpload {
    pub image: TempFile,
    pub style: Text<String>,
    pub furniture: Text<String>,
    pub color: Text<String>,
}

pub async fn generate(
    MultipartForm(form): MultipartForm<ImageUpload>,
) -> Result<HttpResponse, Error> {
    // Acceder a los datos del formulario
    let image = form.image;
    let style = form.style.into_inner();
    let furniture = form.furniture.into_inner();
    let color = form.color.into_inner();

    println!(
        "Datos recibidos: style: {}, furniture: {}, color: {}",
        style, furniture, color
    );

    // Leer la imagen del archivo temporal
    let mut image_data = Vec::new();
    if let Ok(mut file) = std::fs::File::open(image.file) {
        let _ = file.read_to_end(&mut image_data);
    }

    // Decodificar la imagen
    let original_image = match image_service::decode_image(&image_data) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Error al decodificar la imagen: {:?}", e);
            return Ok(HttpResponse::BadRequest().body("Error al decodificar la imagen"));
        }
    };

    // Preprocesar la imagen
    let preprocessed_image = match image_service::preprocess_image(&original_image) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Error al preprocesar la imagen: {:?}", e);
            return Ok(HttpResponse::InternalServerError().body("Error al preprocesar la imagen"));
        }
    };

    // Codificar la imagen preprocesada a bytes
    let preprocessed_image_bytes = match image_service::encode_image(&preprocessed_image) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error al codificar la imagen preprocesada: {:?}", e);
            return Ok(HttpResponse::InternalServerError().body("Error al codificar la imagen"));
        }
    };

    // Construir el prompt para la API de HuggingFace
    let prompt = format!(
        "Estilo: {}, Muebles: {}, Color: {}",
        style, furniture, color
    );

    // Enviar la imagen y el prompt a la API de HuggingFace
    let generated_image_bytes =
        match ai_service::generate_image(preprocessed_image_bytes, prompt).await {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error al generar la imagen con HuggingFace: {:?}", e);
                return Ok(HttpResponse::InternalServerError().body("Error al generar la imagen"));
            }
        };

    // Devolver la imagen generada
    Ok(HttpResponse::Ok()
        .content_type(ContentType::jpeg())
        .body(generated_image_bytes))
}
