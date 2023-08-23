mod auth;
mod favorites;
mod friend_accept;
mod friend_request;
mod friend_status;
mod friends;
mod group;
mod instance;
mod notifications;
mod search_user;
mod toggle_askme;
mod two_factor_email;
mod user;
mod utils;
mod world;

pub(crate) fn route() -> Vec<rocket::Route> {
    routes![
        auth::api_auth,
        favorites::api_add_favorites,
        friend_accept::api_friend_accept,
        friend_request::api_del_friend_request,
        friend_request::api_friend_request,
        friend_status::api_friend_status,
        friends::api_friends,
        group::api_group,
        instance::api_instance,
        notifications::api_notifications,
        search_user::api_search_user,
        toggle_askme::api_check_askme,
        toggle_askme::api_toggle,
        two_factor_email::api_twofactor_email,
        user::api_user,
        world::api_world
    ]
}
