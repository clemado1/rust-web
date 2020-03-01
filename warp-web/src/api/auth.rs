#[macro_export]
macro_rules! join_user {
    () => {
        // $curl -X POST localhost:8000/api/auth/v1 -H "Content-Type: application/json" -d '{ "email": "clemado1@gmail.com", "passwd": "password", "username": "clema", nickname: "do1" }'
        // and $curl 0.0.0.0:8000/api/auth/v1/clemado1@gmail.com
        // To see it created.
        auth_route::join().and_then(auth_handler::join)
    };
}

#[macro_export]
macro_rules! get_user {
    () => {
        // $curl 0.0.0.0.:8000/api/auth/v1/user
        auth_route::get().and_then(auth_handler::get)
    };
}

#[macro_export]
macro_rules! login_user {
    () => {
        // $curl -X PUT localhost:8000/api/auth/v1 -H "Content-Type: application/json" -d '{ "email": "clemado1@gmail.com", "passwd": "password" }'
        // and $curl 0.0.0.0:8000/api/auth/v1/clemado1@gmail.com
        // To get user info
        auth_route::login().and_then(auth_handler::login)
    };
}

#[macro_export]
macro_rules! logout_user {
    () => {
        // $curl -X DELETE localhost:8000/api/auth/v1/clemado1@gmail.com
        auth_route::logout().and_then(auth_handler::logout)
    };
}
