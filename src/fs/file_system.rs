pub struct FileSystem {}

impl FileSystem {
    pub fn read_file(file: &str) -> std::io::Result<String> {
        return std::fs::read_to_string(file);
    }
}