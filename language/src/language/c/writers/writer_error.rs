pub enum WriterError {
    Io(std::io::Error),
}

impl From<std::io::Error> for WriterError {
    fn from(error: std::io::Error) -> Self {
        WriterError::Io(error)
    }
}

impl std::fmt::Display for WriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WriterError::Io(error) => write!(f, "IO error: {}", error),
        }
    }
}

impl std::error::Error for WriterError {}

impl std::fmt::Debug for WriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WriterError::Io(error) => write!(f, "IO error: {:?}", error),
        }
    }
}
