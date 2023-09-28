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

#[macro_export]
macro_rules! get_img {
    ($input:expr, clone) => {
        $crate::general::return_not_empty(
            &$input.userIcon,
            &$input.profilePicOverride,
            &$input.currentAvatarThumbnailImageUrl,
        )
    };
    ($input:expr) => {
        $crate::general::return_not_empty(
            $input.userIcon,
            $input.profilePicOverride,
            $input.currentAvatarThumbnailImageUrl,
        )
    };
}
