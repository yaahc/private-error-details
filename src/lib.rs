use eyre::{eyre, ErrReport};

impl Into<PublicImplHider> for ErrReport {
    fn into(self) -> PublicImplHider {
        PublicImplHider(self)
    }
}

pub struct PublicImplHider(ErrReport);

pub struct Error(PublicImplHider);

impl<T> From<T> for Error
where
    T: Into<PublicImplHider>,
{
    fn from(kind: T) -> Self {
        Error(kind.into())
    }
}

pub fn get(return_error: bool) -> Result<(), Error> {
    if return_error {
        Err(eyre!(
            "heres an error thats implementation is completely hidden from my API"
        ))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
