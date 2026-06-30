//! Use case: create a spray (`.vtf` + `.vmt`) from a raw RGBA image.

use std::sync::Arc;

use crate::domain::entities::{NewSpray, Spray};
use crate::domain::error::{AppError, AppResult};
use crate::domain::repositories::SprayWriter;

pub struct CreateSpray {
    writer: Arc<dyn SprayWriter>,
}

impl CreateSpray {
    pub fn new(writer: Arc<dyn SprayWriter>) -> Self {
        Self { writer }
    }

    pub fn execute(&self, input: NewSpray) -> AppResult<Spray> {
        if input.library_dir.trim().is_empty() {
            return Err(AppError::Validation(
                "no spray library folder configured".into(),
            ));
        }
        let expected = (input.width as usize) * (input.height as usize) * 4;
        if input.rgba.len() != expected {
            return Err(AppError::Validation(format!(
                "image data is {} bytes, expected {expected}",
                input.rgba.len()
            )));
        }
        self.writer.create(&input)
    }
}
