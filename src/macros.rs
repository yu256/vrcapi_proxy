#[macro_export]
macro_rules! split_colon {
    ($input:expr, [$($var:ident),+]) => {
        let mut parts_ = $input.split(':');
        $(
            let $var = parts_.next().ok_or_else(|| anyhow::anyhow!("Failed to split"))?;
        )+
    };
}

#[macro_export] // レスポンス用
macro_rules! into_err {
    ($var:ident) => {
        ApiResponse::Error($var.to_string())
    };
}
