use std::io;

/// This is the `thiserror` error for all crate errors.
///
/// Most ssh related error are wrapped in the `SshError` variant,
/// giving access to the underlying [`russh::Error`] type.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Authentification failed")]
    PasswordWrong,
    #[error("Invalid address was provided")]
    AddressInvalid(io::Error),
    #[error("The executed command didn't send an exit code")]
    CommandDidntExit,
    #[error("Ssh error occured")]
    SshError(#[from] russh::Error),
}
