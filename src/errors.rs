use std::io::Error;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum IOError {
    #[fail(
        display = "There was an error opening input file \"{}\". {}",
        path, context
    )]
    InputFileOpen { path: String, context: Error },

    #[fail(display = "There was an error reading input file. {}", context)]
    InputFileRead { context: Error },

    #[fail(
        display = "There was an error opening output file \"{}\". {}",
        path, context
    )]
    OutputFileOpen { path: String, context: Error },

    #[fail(display = "There was an error writing output file. {}", context)]
    OutputFileWrite { context: Error },
}
