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
mod two_factor;
mod user;
mod utils;
mod world;
mod response;

pub(crate) use friends::{fetch_friends, FRIENDS};
pub(crate) use user::User;

pub(crate) fn route() -> Vec<rocket::Route> {
    routes![
        auth::api_auth,
        favorites::api_add_favorites,
        friend_accept::api_friend_accept,
        friend_request::api_friend_request,
        friend_status::api_friend_status,
        friends::api_friends,
        group::api_group,
        instance::api_instance,
        notifications::api_notifications,
        search_user::api_search_user,
        two_factor::api_twofactor,
        user::api_user,
        world::api_world
    ]
}
