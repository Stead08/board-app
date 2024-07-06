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
pub enum PostsGetResponse {
    /// List of posts
    Status200_ListOfPosts
    (Vec<models::Post>)
    ,
    /// リクエストが不正です
    Status400
    ,
    /// 認証されていません
    Status401
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostsPostResponse {
    /// Post created/updated successfully
    Status201_PostCreated
    (models::Post)
    ,
    /// リクエストが不正です
    Status400
    ,
    /// 認証されていません
    Status401
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostsPostIdDeleteResponse {
    /// No Content
    Status204_NoContent
    ,
    /// Bad Request
    Status400_BadRequest
    ,
    /// Unauthorized
    Status401_Unauthorized
    ,
    /// Not Found
    Status404_NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostsPostIdGetResponse {
    /// Post created/updated successfully
    Status200_PostCreated
    (models::Post)
    ,
    /// リクエストが不正です
    Status400
    ,
    /// 認証されていません
    Status401
    ,
    /// 投稿が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostsPostIdPutResponse {
    /// Post created/updated successfully
    Status200_PostCreated
    (models::Post)
    ,
    /// リクエストが不正です
    Status400
    ,
    /// 認証されていません
    Status401
    ,
    /// 投稿が見つかりません
    Status404
}


/// Posts
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Posts {
    /// すべての投稿を取得.
    ///
    /// PostsGet - GET /posts
    async fn posts_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      header_params: models::PostsGetHeaderParams,
    ) -> Result<PostsGetResponse, String>;

    /// 新規投稿作成.
    ///
    /// PostsPost - POST /posts
    async fn posts_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      header_params: models::PostsPostHeaderParams,
            body: Option<models::PostsPostRequest>,
    ) -> Result<PostsPostResponse, String>;

    /// 投稿を削除.
    ///
    /// PostsPostIdDelete - DELETE /posts/{postId}
    async fn posts_post_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      header_params: models::PostsPostIdDeleteHeaderParams,
      path_params: models::PostsPostIdDeletePathParams,
    ) -> Result<PostsPostIdDeleteResponse, String>;

    /// IDで投稿を取得.
    ///
    /// PostsPostIdGet - GET /posts/{postId}
    async fn posts_post_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      header_params: models::PostsPostIdGetHeaderParams,
      path_params: models::PostsPostIdGetPathParams,
    ) -> Result<PostsPostIdGetResponse, String>;

    /// 投稿を更新.
    ///
    /// PostsPostIdPut - PUT /posts/{postId}
    async fn posts_post_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      header_params: models::PostsPostIdPutHeaderParams,
      path_params: models::PostsPostIdPutPathParams,
            body: Option<models::Post>,
    ) -> Result<PostsPostIdPutResponse, String>;
}
