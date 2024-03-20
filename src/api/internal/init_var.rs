use crate::{
    api::favorites::fetch_favorite_friends,
    global::{FRIENDS, MYSELF},
    internal::fetch_friends::fetch_all_friends,
    user::User,
};

use super::user_info::fetch_user_info;

pub(crate) async fn init_var(token: &str) -> anyhow::Result<()> {
    tokio::try_join!(
        init_myself_and_friends(token),
        fetch_favorite_friends(token)
    )?;

    Ok(())
}

async fn init_myself_and_friends(token: &str) -> anyhow::Result<()> {
    let (user_profile, online, offline) = tokio::try_join!(
        fetch_user_info(token),
        fetch_all_friends(token, false),
        fetch_all_friends(token, true),
    )?;

    let (online, web) = online
        .into_iter()
        .partition(|u| user_profile.activeFriends.contains(&u.id));

    tokio::join!(
        async {
            let mut friends = FRIENDS.write().await;
            friends.online = online;
            friends.web = web;
            friends.offline = offline;
        },
        async {
            let presence = user_profile.presence;
            let user = User {
                bio: user_profile.bio,
                bioLinks: user_profile.bioLinks,
                currentAvatarThumbnailImageUrl: user_profile.currentAvatarThumbnailImageUrl,
                displayName: user_profile.displayName,
                id: user_profile.id,
                isFriend: user_profile.isFriend,
                location: if presence.world.is_empty() || presence.instance.is_empty() {
                    String::new()
                } else {
                    format!("{}:{}", presence.world, presence.instance)
                },
                travelingToLocation: (!presence.travelingToWorld.is_empty()
                    && !presence.travelingToInstance.is_empty())
                .then(|| {
                    format!(
                        "{}:{}",
                        presence.travelingToWorld, presence.travelingToInstance
                    )
                }),
                status: user_profile.status,
                statusDescription: user_profile.statusDescription,
                tags: user_profile.tags,
                userIcon: user_profile.userIcon,
                profilePicOverride: user_profile.profilePicOverride,
            };
            MYSELF.insert(user).await;
        }
    );

    Ok(())
}