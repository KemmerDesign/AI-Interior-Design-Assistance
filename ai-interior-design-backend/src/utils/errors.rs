use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    ImageDecodingError(String),
    ImageProcessingError(String),
    HuggingFaceError(String),
    // ... otros errores
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ImageDecodingError(msg) => write!(f, "Error al decodificar la imagen: {}", msg),
            AppError::ImageProcessingError(msg) => write!(f, "Error al procesar la imagen: {}", msg),
            AppError::HuggingFaceError(msg) => write!(f, "Error al comunicarse con HuggingFace: {}", msg),
            // ... otros errores
        }
    }
}

impl Error for AppError {}

// Puedes implementar conversiones desde otros tipos de error a AppError, por ejemplo:
impl From<opencv::Error> for AppError {
    fn from(err: opencv::Error) -> Self {
        AppError::ImageProcessingError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HuggingFaceError(err.to_string())
    }
}

// ... y así sucesivamente para otros errores que puedas encontrar