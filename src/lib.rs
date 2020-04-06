#[derive(Debug, thiserror::Error, displaydoc::Display)]
enum Kind {
    /// Encountered the only error case this library supports
    OnlyError,
}

impl Into<PublicImplHider> for Kind {
    fn into(self) -> PublicImplHider {
        PublicImplHider(self)
    }
}

pub struct PublicImplHider(Kind);

pub struct Error(PublicImplHider);

impl<T> From<T> for Error
where
    T: Into<PublicImplHider>,
{
    fn from(kind: T) -> Self {
        Error(kind.into())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        (self.0).0.source()
    }
}

use std::fmt::{self, Debug, Display, Formatter};

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&(self.0).0, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&(self.0).0, f)
    }
}

pub fn get(return_error: bool) -> Result<(), Error> {
    if return_error {
        Err(Kind::OnlyError)?;
    }

    Ok(())
}
