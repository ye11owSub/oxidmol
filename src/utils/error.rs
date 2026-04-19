use thiserror::Error;

#[derive(Error, Debug)]
#[error("Unknown element symbol: {0:?}")]
pub struct UnknownElement(pub String);

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("PDB parse failed: {0}")]
    Pdb(String),
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_wgpu_error_display() {
        let error = WgpuError::SurfaceCreation("Test error".to_string());
        assert_eq!(error.to_string(), "Failed to create surface: Test error");
    }

    #[test]
    fn test_no_adapter_error() {
        let error = WgpuError::NoAdapter;
        assert_eq!(error.to_string(), "No suitable adapter found");
    }

    #[test]
    fn test_device_creation_error() {
        let error = WgpuError::DeviceCreation("Device not available".to_string());
        assert_eq!(
            error.to_string(),
            "Device creation failed: Device not available"
        );
    }

    #[test]
    fn test_texture_loading_error() {
        let error = WgpuError::TextureLoading("Invalid format".to_string());
        assert_eq!(error.to_string(), "Texture loading failed: Invalid format");
    }

    #[test]
    fn test_shader_compilation_error() {
        let error = WgpuError::ShaderCompilation("Syntax error on line 10".to_string());
        assert_eq!(
            error.to_string(),
            "Shader compilation failed: Syntax error on line 10"
        );
    }

    #[test]
    fn test_io_error_from_conversion() {
        let io_error = IoError::new(ErrorKind::NotFound, "File not found");
        let wgpu_error = WgpuError::from(io_error);

        match wgpu_error {
            WgpuError::Io(_) => {}
            _ => panic!("Expected WgpuError::Io variant"),
        }
    }

    #[test]
    fn test_error_debug_format() {
        let error = WgpuError::NoAdapter;
        let debug_str = format!("{:?}", error);
        assert_eq!(debug_str, "NoAdapter");
    }

    #[test]
    fn test_error_chain() {
        let io_error = IoError::new(ErrorKind::PermissionDenied, "Access denied");
        let wgpu_error = WgpuError::from(io_error);

        assert!(wgpu_error.source().is_some());
    }
}
