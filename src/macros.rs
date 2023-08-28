#[macro_export]
macro_rules! split_colon {
    ($input:expr, [$($var:ident),+]) => {
        let mut parts_ = $input.split(':');
        $(
            let $var = parts_.next().ok_or(anyhow::anyhow!("Failed to split"))?;
        )+
    };
}
