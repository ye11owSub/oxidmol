use thiserror::Error;

#[derive(Error, Debug)]
pub enum WgpuError {
    #[error("Failed to create surface: {0}")]
    SurfaceCreation(String),
    
    #[error("No suitable adapter found")]
    NoAdapter,
    
    #[error("Device creation failed: {0}")]
    DeviceCreation(String),
    
    #[error("Texture loading failed: {0}")]
    TextureLoading(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilation(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

