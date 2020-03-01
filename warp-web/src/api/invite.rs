#[macro_export]
macro_rules! invite_user {
    () => {
        // $curl -X POST localhost:8000/invite/auth/v1 -H "Content-Type: application/json" -d '{ "email": "clemado1@gmail.com"}'
        invite_route::invite().and_then(invite_handler::invite)
    };
}

#[macro_export]
macro_rules! authenticate_user {
    () => {
        // $curl 0.0.0.0.:8000/api/invite/v1/session
        invite_route::authenticate().and_then(invite_handler::authenticate)
    };
}
