#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::{
        filters,
        models::{self, Todo},
    };

    #[tokio::test]
    async fn test_post() {
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/api/auth")
            .json(&Todo {
                id: 1,
                text: "test 1".into(),
                completed: false,
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_post_conflict() {
        let db = models::blank_db();
        db.lock().await.push(todo1());
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/api/auth")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_put_unknown() {
        let _ = pretty_env_logger::try_init();
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("PUT")
            .path("/api/auth")
            .header("authorization", "Bearer admin")
            .json(&user())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    fn user() -> Todo {
        Todo {
            id: 1,
            text: "test 1".into(),
            completed: false,
        }
    }
}
