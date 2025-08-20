use crate::common::cache_local::CacheLocalRequestExt;
use crate::common::config::Config;
use crate::common::context::{Context, ContextError, Dep, FromContext};
use crate::user::model::UserIdContext;
use crate::user::service::UserCheckService;
use error_stack::Report;
use poem::http::StatusCode;
use poem::{FromRequest, Request, RequestBody};
use std::marker::PhantomData;
use std::sync::{Arc, Weak};

pub trait FromUserContext: Sized + Send + Sync {
    fn from_user_context(
        ctx: &'_ UserContext,
    ) -> impl Future<Output = Result<Self, Report<ContextError>>> + Send;
}

pub struct UserContext<'a> {
    pub user_context: Arc<UserIdContext>,
    pub config: Weak<Config>,
    pub req: &'a Request,
}

impl UserContext<'_> {
    pub async fn inject<T: FromUserContext>(&self) -> Result<T, Report<ContextError>> {
        T::from_user_context(self).await
    }

    pub async fn inject_global<T: FromContext>(&self) -> Result<T, Report<ContextError>> {
        let ctx = Box::pin(Context {
            config: Weak::clone(&self.config),
            req: self.req,
        });
        T::from_context(&ctx).await
    }
}

pub struct UserContextFlagData {
    pub allow_user: bool,
    pub allow_visitor: bool,
}

pub trait UserContextDependencyFlag: Sized + Send + Sync {
    const ALLOW_USER: bool = true;
    const ALLOW_VISITOR: bool = true;

    fn build_flag_data() -> UserContextFlagData {
        UserContextFlagData {
            allow_user: Self::ALLOW_USER,
            allow_visitor: Self::ALLOW_VISITOR,
        }
    }
}

pub struct DefaultFlag;

impl UserContextDependencyFlag for DefaultFlag {}

pub struct UserDependency<T, F = DefaultFlag>(pub T, PhantomData<F>)
where
    T: FromUserContext,
    F: UserContextDependencyFlag;

pub type UserDep<T, F = DefaultFlag> = UserDependency<T, F>;

impl<'a, T, F> FromRequest<'a> for UserDependency<T, F>
where
    T: FromUserContext,
    F: UserContextDependencyFlag,
{
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> poem::Result<Self> {
        let config = match Config::fetch().await {
            Ok(config) => config,
            Err(_) => return Err(poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
        };
        let flag = F::build_flag_data();
        let user_id_context = match req.cache_local::<Arc<UserIdContext>>() {
            None => {
                return Err(poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
            }
            Some(once_user_id_context) => {
                let v: Result<&Arc<UserIdContext>, poem::Error> = once_user_id_context
                    .0
                    .get_or_try_init(|| async {
                        let user_service: Dep<UserCheckService> =
                            Dep::from_request(req, body).await?;

                        Ok(Arc::new(user_service.0.get_user_context()))
                    })
                    .await;
                let v = v?;
                Arc::clone(&v)
            }
        };

        if user_id_context.is_user && !flag.allow_user {
            return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
        } else if !user_id_context.is_user && !flag.allow_visitor {
            return Err(poem::Error::from_status(StatusCode::FORBIDDEN));
        }

        let context = Box::pin(UserContext {
            user_context: Arc::clone(&user_id_context),
            config,
            req,
        });
        Ok(Self(
            T::from_user_context(&context).await.map_err(|e| {
                let status_code = e.current_context().status_code();
                poem::Error::from_string(status_code.1, status_code.0)
            })?,
            PhantomData,
        ))
    }
}
