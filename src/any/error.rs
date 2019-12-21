//! Error and result types for `tree::any` module.

use std::{error, fmt};

use fbxcel::{low::FbxVersion, tree};

/// AnyTree load result.
pub type Result<T> = std::result::Result<T, Error>;

/// Error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Unknown FBX parser.
    ///
    /// This means that the FBX version may be supported by the backend parser, but the backend
    /// parser used to load the document is unsupported by fbxcel-dom crate.
    UnsupportedVersion(FbxVersion),
    /// Tree load error.
    Tree(tree::any::Error),
    /// DOM load error.
    Dom(Box<dyn error::Error + Send + Sync + 'static>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Tree(e) => Some(e),
            Error::Dom(e) => Some(&**e),
            Error::UnsupportedVersion(..) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Tree(e) => write!(f, "Tree load error: {}", e),
            Error::Dom(e) => write!(f, "DOM document load error: {}", e),
            Error::UnsupportedVersion(ver) => write!(f, "Unsupported FBX version: {:?}", ver),
        }
    }
}

impl From<tree::any::Error> for Error {
    fn from(e: tree::any::Error) -> Self {
        Error::Tree(e)
    }
}

impl From<crate::v7400::LoadError> for Error {
    fn from(e: crate::v7400::LoadError) -> Self {
        Error::Dom(e.into())
    }
}
