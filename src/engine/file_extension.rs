use std::path::Path;

enum FileExtensions {
    MUSH,
    MSH,
    MS,
}

impl FileExtensions {
    fn from_string(value: &str) -> Option<Self> {
        match value {
            "mush" => Some(Self::MUSH),
            "msh" => Some(Self::MSH),
            "ms" => Some(Self::MS),
            _ => None,
        }
    }
}

pub fn has_valid_file_extension(file_path: &Path) -> bool {
    if let Some(extension) = file_path.extension().and_then(|e| e.to_str()) {
        return FileExtensions::from_string(extension).is_some();
    }
    return false;
}
