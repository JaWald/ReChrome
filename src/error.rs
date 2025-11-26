use std::path::PathBuf;
use std::time::SystemTimeError;
use image::ImageError;

pub enum AppError {
    InputFileDoesNotExist(PathBuf),
    ImageError(ImageError),
    SystemTimeError(SystemTimeError),
    IoError(std::io::Error),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::IoError(err)
    }
}

impl From<ImageError> for AppError {
    fn from(err: ImageError) -> AppError {
        AppError::ImageError(err)
    }
}

impl From<SystemTimeError> for AppError {
    fn from(err: SystemTimeError) -> AppError {
        AppError::SystemTimeError(err)
    }
}

