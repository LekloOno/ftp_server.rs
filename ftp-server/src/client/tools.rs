/// A quick hand to simplify implementations of commands that require authentication.
/// 
/// It checks if the provided client is authentified, and if not, sends a `NotLoggedIn`
/// response through the control stream and early returns `Ok(())`.
#[macro_export]
macro_rules! check_auth {
    ($client:expr) => {
        if !$client.authentified {
            crate::response::perma_error::PermanentError::NotLoggedIn { details: String::new() }.send($client.control())?;
            return Ok(());
        }
    };
}