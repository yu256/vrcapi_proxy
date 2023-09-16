#[macro_export]
macro_rules! split_colon {
    ($input:expr, [$($var:ident),+]) => {
        let mut parts_ = $input.split(':');
        $(
            let Some($var) = parts_.next() else {
                anyhow::bail!("Failed to split");
            };
        )+
    };
}
