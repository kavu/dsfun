use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IOError {
    #[error("There was an error opening input file \"{path:?}\".")]
    InputFileOpen { path: String, source: io::Error },

    #[error("There was an error reading input file.")]
    InputFileRead { source: io::Error },

    #[error("There was an error opening output file \"{path:?}\".")]
    OutputFileOpen { path: String, source: io::Error },

    #[error("There was an error writing output file.")]
    OutputFileWrite { source: io::Error },
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("There was an error during processing.")]
    ProcessingError { source: IOError },
}

impl From<IOError> for AppError {
    fn from(err: IOError) -> Self {
        Self::ProcessingError { source: err }
    }
}
