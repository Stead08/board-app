mod entity;
mod service;
mod value_object;

use crate::entity::User;
use crate::service::jwt;
use crate::value_object::PostId;
use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::apis::posts::PostsPostIdDeleteResponse;
use openapi::models::{
    Post, PostsGetHeaderParams, PostsPostHeaderParams, PostsPostIdDeleteHeaderParams,
    PostsPostIdDeletePathParams, PostsPostIdGetHeaderParams, PostsPostIdGetPathParams,
    PostsPostIdPutHeaderParams, PostsPostIdPutPathParams,
};
use openapi::server::new;
use openapi::{
    apis::{
        auth::{Auth, AuthPostResponse},
        posts::{
            Posts, PostsGetResponse, PostsPostIdGetResponse, PostsPostIdPutResponse,
            PostsPostResponse,
        },
        users::{Users, UsersPostResponse},
    },
    models,
    types::*,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::log::log;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use validator::Validate;

const SECRET: &str = "secret";

#[derive(Clone)]
struct ApiImpl {
    users: Arc<Mutex<Vec<entity::User>>>,
    posts: Arc<Mutex<Vec<entity::Post>>>,
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        &self
    }
}

#[async_trait]
impl Users for ApiImpl {
    async fn users_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::UsersPostRequest>,
    ) -> Result<UsersPostResponse, String> {
        let body = body.ok_or("body is required")?;
        body.validate().map_err(|e| e.to_string())?;
        // todo! idの生成方法を考える
        // user idはusersの長さ+1
        let user = User {
            id: value_object::UserId::from(self.users.lock().unwrap().len() as i64 + 1),
            name: value_object::Name::from(body.name.clone()),
            email: value_object::Email::from(body.email.clone()),
            password: value_object::Password::from(body.password),
        };

        let mut users = self.users.lock().unwrap();
        users.push(user.clone());

        Ok(UsersPostResponse::Status201_UserCreatedSuccessfully(
            models::User {
                id: Some(user.id),
                name: Some(body.name),
                email: Some(body.email),
                password: None,
            },
        ))
    }
}

#[async_trait]
impl Posts for ApiImpl {
    async fn posts_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        header_params: PostsGetHeaderParams,
    ) -> Result<PostsGetResponse, String> {
        let jwt = header_params.authorization.replace("Bearer ", "");
        match jwt::validate_token(&SECRET.as_ref(), &jwt) {
            Ok(_) => {}
            Err(_) => return Ok(PostsGetResponse::Status401),
        };
        let posts_locked = self.posts.lock().unwrap();
        let posts = posts_locked
            .iter()
            .map(|post| models::Post {
                id: Some(post.id),
                title: Some(post.title.clone()),
                content: Some(post.content.clone()),
                user_id: Some(post.user_id),
            })
            .collect();
        Ok(PostsGetResponse::Status200_ListOfPosts(posts))
    }

    async fn posts_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        header_params: PostsPostHeaderParams,
        body: std::option::Option<openapi::models::PostsPostRequest>,
    ) -> Result<PostsPostResponse, String> {
        let jwt = header_params.authorization.replace("Bearer ", "");
        let Ok(jwt) = jwt::validate_token(&SECRET.as_ref(), &jwt) else {
            return Ok(PostsPostResponse::Status401);
        };
        let body = body.ok_or("body is required")?;
        body.validate().map_err(|e| e.to_string())?;
        let post = entity::Post {
            id: PostId::new_v4(),
            user_id: jwt.uid.parse().unwrap(),
            title: body.title.clone(),
            content: body.content.clone(),
        };
        let mut posts_locked = self.posts.lock().unwrap();
        posts_locked.push(post.clone());
        Ok(PostsPostResponse::Status201_PostCreated(models::Post {
            id: Some(post.id),
            title: Some(post.title),
            content: Some(post.content),
            user_id: Some(post.user_id),
        }))
    }

    async fn posts_post_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        header_params: PostsPostIdDeleteHeaderParams,
        path_params: PostsPostIdDeletePathParams,
    ) -> Result<PostsPostIdDeleteResponse, String> {
        let jwt = header_params.authorization.replace("Bearer ", "");
        let Ok(jwt) = jwt::validate_token(&SECRET.as_ref(), &jwt) else {
            return Ok(PostsPostIdDeleteResponse::Status401_Unauthorized);
        };
        let post_id = path_params.post_id;
        let mut posts_locked = self.posts.lock().unwrap();
        let post = posts_locked.iter().find(|post| post.id == post_id);
        if let Some(post) = post {
            if post.user_id == jwt.uid.parse::<i64>().unwrap() {
                posts_locked.retain(|post| post.id != post_id);
                Ok(PostsPostIdDeleteResponse::Status204_NoContent)
            } else {
                Ok(PostsPostIdDeleteResponse::Status401_Unauthorized)
            }
        } else {
            Ok(PostsPostIdDeleteResponse::Status404_NotFound)
        }
    }

    async fn posts_post_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: PostsPostIdGetHeaderParams,
        path_params: PostsPostIdGetPathParams,
    ) -> Result<PostsPostIdGetResponse, String> {
        let jwt = header_params.authorization.replace("Bearer ", "");
        let Ok(jwt) = jwt::validate_token(&SECRET.as_ref(), &jwt) else {
            return Ok(PostsPostIdGetResponse::Status401);
        };
        let post_id = path_params.post_id;
        let posts_locked = self.posts.lock().unwrap();
        let post = posts_locked.iter().find(|post| post.id == post_id);
        if let Some(post) = post {
            Ok(PostsPostIdGetResponse::Status200_PostCreated(
                models::Post {
                    id: Some(post.id),
                    title: Some(post.title.clone()),
                    content: Some(post.content.clone()),
                    user_id: Some(post.user_id),
                },
            ))
        } else {
            Ok(PostsPostIdGetResponse::Status404)
        }
    }

    async fn posts_post_id_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: PostsPostIdPutHeaderParams,
        path_params: PostsPostIdPutPathParams,
        body: Option<Post>,
    ) -> Result<PostsPostIdPutResponse, String> {
        let jwt = header_params.authorization.replace("Bearer ", "");
        let Ok(jwt) = jwt::validate_token(&SECRET.as_ref(), &jwt) else {
            return Ok(PostsPostIdPutResponse::Status401);
        };
        let post_id = path_params.post_id;
        let body = body.ok_or("body is required")?;
        body.validate().map_err(|e| e.to_string())?;
        let mut posts_locked = self.posts.lock().unwrap();
        let post = posts_locked.iter_mut().find(|post| post.id == post_id);
        if let Some(post) = post {
            if post.user_id == jwt.uid.parse::<i64>().unwrap() {
                post.title = body.title.clone().unwrap();
                post.content = body.content.clone().unwrap();
                Ok(PostsPostIdPutResponse::Status200_PostCreated(
                    models::Post {
                        id: Some(post.id),
                        title: Some(post.title.clone()),
                        content: Some(post.content.clone()),
                        user_id: Some(post.user_id),
                    },
                ))
            } else {
                Ok(PostsPostIdPutResponse::Status401)
            }
        } else {
            Ok(PostsPostIdPutResponse::Status404)
        }
    }
}

#[async_trait]
impl Auth for ApiImpl {
    async fn auth_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::Auth>,
    ) -> Result<AuthPostResponse, String> {
        let (email, password) = match body {
            Some(body) => (body.email, body.password),
            None => return Err("body is required".to_string()),
        };
        let users_locked = self.users.lock().unwrap();
        // todo! ハッシュ化したパスワードを利用する
        let user = users_locked.iter().find(|user| {
            user.email == *email.as_ref().unwrap() && user.password == *password.as_ref().unwrap()
        });
        if let Some(user) = user {
            let token = jwt::create_token(SECRET.as_ref(), &user.id.to_string())
                .map_err(|e| e.to_string())?;
            Ok(AuthPostResponse::Status200_AuthenticationSuccessful(
                models::Token { token: Some(token) },
            ))
        } else {
            Ok(AuthPostResponse::Status400)
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let users = Arc::new(Mutex::new(Vec::new()));
    let posts = Arc::new(Mutex::new(Vec::new()));
    let api = ApiImpl { users, posts };
    let router = new(api);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind to address");
    axum::serve(listener, router).await.unwrap();
}
