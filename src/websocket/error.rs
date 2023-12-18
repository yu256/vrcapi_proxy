#[derive(Debug)]
pub(crate) enum WSError {
	UnknownErr,
    TokenErr,
    OtherErr(String),
}