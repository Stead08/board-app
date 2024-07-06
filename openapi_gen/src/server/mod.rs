use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};


/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::auth::Auth + apis::posts::Posts + apis::users::Users + 'static,
{
    // build our application with a route
    Router::new()
        .route("/auth",
            post(auth_post::<I, A>)
        )
        .route("/posts",
            get(posts_get::<I, A>).post(posts_post::<I, A>)
        )
        .route("/posts/:post_id",
            delete(posts_post_id_delete::<I, A>).get(posts_post_id_get::<I, A>).put(posts_post_id_put::<I, A>)
        )
        .route("/users",
            post(users_post::<I, A>)
        )
        .with_state(api_impl)
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct AuthPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Auth,
    }


#[tracing::instrument(skip_all)]
fn auth_post_validation(
        body: Option<models::Auth>,
) -> std::result::Result<(
        Option<models::Auth>,
), ValidationErrors>
{
            if let Some(body) = &body {
              let b = AuthPostBodyValidator { body };
              b.validate()?;
            }

Ok((
    body,
))
}
/// AuthPost - POST /auth
#[tracing::instrument(skip_all)]
async fn auth_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::Auth>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::auth::Auth,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    auth_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().auth_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::auth::AuthPostResponse::Status200_AuthenticationSuccessful
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::auth::AuthPostResponse::Status400
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn posts_get_validation(
  header_params: models::PostsGetHeaderParams,
) -> std::result::Result<(
  models::PostsGetHeaderParams,
), ValidationErrors>
{
  header_params.validate()?;

Ok((
  header_params,
))
}
/// PostsGet - GET /posts
#[tracing::instrument(skip_all)]
async fn posts_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::posts::Posts,
{
    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header Authorization")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::PostsGetHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    posts_get_validation(
        header_params,
    )
  ).await.unwrap();

  let Ok((
    header_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().posts_get(
      method,
      host,
      cookies,
        header_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::posts::PostsGetResponse::Status200_ListOfPosts
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::posts::PostsGetResponse::Status400
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsGetResponse::Status401
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::PostsPostRequest,
    }


#[tracing::instrument(skip_all)]
fn posts_post_validation(
  header_params: models::PostsPostHeaderParams,
        body: Option<models::PostsPostRequest>,
) -> std::result::Result<(
  models::PostsPostHeaderParams,
        Option<models::PostsPostRequest>,
), ValidationErrors>
{
  header_params.validate()?;
            if let Some(body) = &body {
              let b = PostsPostBodyValidator { body };
              b.validate()?;
            }

Ok((
  header_params,
    body,
))
}
/// PostsPost - POST /posts
#[tracing::instrument(skip_all)]
async fn posts_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::PostsPostRequest>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::posts::Posts,
{
    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header Authorization")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::PostsPostHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    posts_post_validation(
        header_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().posts_post(
      method,
      host,
      cookies,
        header_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::posts::PostsPostResponse::Status201_PostCreated
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::posts::PostsPostResponse::Status400
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostResponse::Status401
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn posts_post_id_delete_validation(
  header_params: models::PostsPostIdDeleteHeaderParams,
  path_params: models::PostsPostIdDeletePathParams,
) -> std::result::Result<(
  models::PostsPostIdDeleteHeaderParams,
  models::PostsPostIdDeletePathParams,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;

Ok((
  header_params,
  path_params,
))
}
/// PostsPostIdDelete - DELETE /posts/{postId}
#[tracing::instrument(skip_all)]
async fn posts_post_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::PostsPostIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::posts::Posts,
{
    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header Authorization")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::PostsPostIdDeleteHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    posts_post_id_delete_validation(
        header_params,
        path_params,
    )
  ).await.unwrap();

  let Ok((
    header_params,
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().posts_post_id_delete(
      method,
      host,
      cookies,
        header_params,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::posts::PostsPostIdDeleteResponse::Status204_NoContent
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostIdDeleteResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostIdDeleteResponse::Status401_Unauthorized
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostIdDeleteResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn posts_post_id_get_validation(
  header_params: models::PostsPostIdGetHeaderParams,
  path_params: models::PostsPostIdGetPathParams,
) -> std::result::Result<(
  models::PostsPostIdGetHeaderParams,
  models::PostsPostIdGetPathParams,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;

Ok((
  header_params,
  path_params,
))
}
/// PostsPostIdGet - GET /posts/{postId}
#[tracing::instrument(skip_all)]
async fn posts_post_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::PostsPostIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::posts::Posts,
{
    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header Authorization")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::PostsPostIdGetHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    posts_post_id_get_validation(
        header_params,
        path_params,
    )
  ).await.unwrap();

  let Ok((
    header_params,
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().posts_post_id_get(
      method,
      host,
      cookies,
        header_params,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::posts::PostsPostIdGetResponse::Status200_PostCreated
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::posts::PostsPostIdGetResponse::Status400
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostIdGetResponse::Status401
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostIdGetResponse::Status404
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostsPostIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Post,
    }


#[tracing::instrument(skip_all)]
fn posts_post_id_put_validation(
  header_params: models::PostsPostIdPutHeaderParams,
  path_params: models::PostsPostIdPutPathParams,
        body: Option<models::Post>,
) -> std::result::Result<(
  models::PostsPostIdPutHeaderParams,
  models::PostsPostIdPutPathParams,
        Option<models::Post>,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;
            if let Some(body) = &body {
              let b = PostsPostIdPutBodyValidator { body };
              b.validate()?;
            }

Ok((
  header_params,
  path_params,
    body,
))
}
/// PostsPostIdPut - PUT /posts/{postId}
#[tracing::instrument(skip_all)]
async fn posts_post_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::PostsPostIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::Post>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::posts::Posts,
{
    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header Authorization")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::PostsPostIdPutHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    posts_post_id_put_validation(
        header_params,
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().posts_post_id_put(
      method,
      host,
      cookies,
        header_params,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::posts::PostsPostIdPutResponse::Status200_PostCreated
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::posts::PostsPostIdPutResponse::Status400
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostIdPutResponse::Status401
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                                apis::posts::PostsPostIdPutResponse::Status404
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct UsersPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::UsersPostRequest,
    }


#[tracing::instrument(skip_all)]
fn users_post_validation(
        body: Option<models::UsersPostRequest>,
) -> std::result::Result<(
        Option<models::UsersPostRequest>,
), ValidationErrors>
{
            if let Some(body) = &body {
              let b = UsersPostBodyValidator { body };
              b.validate()?;
            }

Ok((
    body,
))
}
/// UsersPost - POST /users
#[tracing::instrument(skip_all)]
async fn users_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::UsersPostRequest>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::users::Users,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    users_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().users_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::users::UsersPostResponse::Status201_UserCreatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::users::UsersPostResponse::Status400
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

