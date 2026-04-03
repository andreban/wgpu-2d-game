#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Failed to create surface: {0}")]
    Surface(#[from] wgpu::CreateSurfaceError),
    #[error("No suitable GPU adapter found: {0}")]
    AdapterNotFound(#[from] wgpu::RequestAdapterError),
    #[error("Failed to request device: {0}")]
    Device(#[from] wgpu::RequestDeviceError),
    #[error("Failed to load texture: {0}")]
    Texture(#[from] image::ImageError),
}
