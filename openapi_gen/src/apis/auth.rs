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
pub enum AuthPostResponse {
    /// Authentication successful, token returned
    Status200_AuthenticationSuccessful
    (models::Token)
    ,
    /// リクエストが不正です
    Status400
}


/// Auth
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Auth {
    /// ユーザー認証.
    ///
    /// AuthPost - POST /auth
    async fn auth_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: Option<models::Auth>,
    ) -> Result<AuthPostResponse, String>;
}
