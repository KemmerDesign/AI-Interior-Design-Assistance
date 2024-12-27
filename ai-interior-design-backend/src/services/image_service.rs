use base64::{engine::general_purpose, Engine as _};
use opencv::{
    core::{self, Mat, Size, Vector},
    imgcodecs, imgproc,
    prelude::VectorToVec,
};
use std::error::Error;
use opencv::prelude::MatTraitConst;

pub fn decode_image(image_data: &[u8]) -> Result<Mat, Box<dyn Error>> {
    // Decode the image bytes using imdecode
    let decoded_image =
        imgcodecs::imdecode(&Mat::from_slice(image_data)?, imgcodecs::IMREAD_COLOR)?;

    Ok(decoded_image)
}

pub fn preprocess_image(image: &Mat) -> Result<Mat, Box<dyn Error>> {
    // 1. Redimensionar la imagen
    let mut resized_image = Mat::default();
    imgproc::resize(
        image,
        &mut resized_image,
        Size::new(512, 512), // Ajusta el tamaño según los requerimientos del modelo de HuggingFace
        0.0,
        0.0,
        imgproc::INTER_AREA,
    )?;

    // 2. Convertir a escala de grises (si es necesario)
    // let mut gray_image = Mat::default();
    // imgproc::cvt_color(&resized_image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0)?;

    // 3. (Opcional) Segmentación, detección de bordes, etc. (implementar según se necesite)

    Ok(resized_image)
}

pub fn encode_image(image: &Mat) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = Vector::<u8>::new();
    imgcodecs::imencode(".jpg", image, &mut buffer, &core::Vector::new())?;
    Ok(buffer.to_vec())
}

pub fn decode_base64_to_image(image_data: &str) -> Result<Mat, Box<dyn Error>> {
    // Decodifica la cadena base64 a bytes
    let image_bytes = general_purpose::STANDARD.decode(image_data)?;

    // Decodifica los bytes de la imagen a un objeto Mat
    decode_image(&image_bytes)
}