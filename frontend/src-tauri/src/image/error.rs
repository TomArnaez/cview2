use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageManagerError {
    #[error("image not found error")]
    ImageNotFound
}
