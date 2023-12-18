#[macro_export]
macro_rules! split_colon {
    ($input:expr, [$($var:ident),+]) => {
		let mut iter_ = $input.split(':');
        $(
            let Some($var) = iter_.next() else {
                anyhow::bail!($crate::global::INVALID_REQUEST);
            };
        )+
    };
}

/// If the second argument is given, it becomes the token. If not, auth is returned.
#[macro_export]
macro_rules! validate {
    ($auth:expr, $token:ident) => {
        let (auth_, ref $token) = {
            let auth = &$crate::global::AUTHORIZATION;
            (auth.0, auth.1.read().await)
        };
        anyhow::ensure!($auth == auth_, $crate::global::INVALID_AUTH);
    };
    ($auth_:expr) => {{
        let auth = $crate::global::AUTHORIZATION.0;
        anyhow::ensure!($auth_ == auth, $crate::global::INVALID_AUTH);
        auth
    }};
}

#[macro_export]
macro_rules! struct_foreach {
    ($struct_name:ident, [$($field:ident),*], $operation:ident($operation_arg:expr)) => {
        $(
            $struct_name.$field.$operation($operation_arg);
        )*
    };
}
