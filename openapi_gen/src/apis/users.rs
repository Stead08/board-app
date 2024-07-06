use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersPostResponse {
    /// User created successfully
    Status201_UserCreatedSuccessfully
    (models::User)
    ,
    /// リクエストが不正です
    Status400
}


/// Users
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Users {
    /// 新規ユーザー登録.
    ///
    /// UsersPost - POST /users
    async fn users_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: Option<models::UsersPostRequest>,
    ) -> Result<UsersPostResponse, String>;
}
