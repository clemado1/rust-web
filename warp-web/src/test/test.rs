use warp::Filter;

use crate::{
    authenticate_user, get_user,
    handlers::{auth_handler, invite_hanlder},
    invite_user, join_user, login_user, logout_user,
    routes::{auth_route, invite_route},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_user() {
        let res = warp::test::request()
            .method("GET")
            .path("/api/auth/v1/clemado1")
            .reply(&get_user)
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK");
        println!("{:#?}", res.body());
    }
}

// authenticate_user ...
