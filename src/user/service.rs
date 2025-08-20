use crate::common::context::{Context, ContextError, FromContext};
use crate::user::model::{IdUsername, UserIdContext};
use crate::user::repository::UserRepository;
use error_stack::Report;

pub struct UserCheckService {
    user_repository: UserRepository,
    token_cookie: Option<String>,
}

impl UserCheckService {
    pub fn new(user_repository: UserRepository, token_cookie: Option<String>) -> Self {
        Self {
            user_repository,
            token_cookie,
        }
    }

    pub fn get_user_context(&self) -> UserIdContext {
        if let Some(id_username) = self.is_logged_in() {
            UserIdContext {
                id: id_username.id,
                is_user: true,
                username: id_username.username,
            }
        } else {
            UserIdContext {
                id: 0,
                is_user: false,
                username: "Visitor".to_string(),
            }
        }
    }

    fn is_logged_in(&self) -> Option<IdUsername> {
        if let Some(token) = &self.token_cookie {
            if let Ok(id_username) = self.user_repository.find_by_token(token.clone()) {
                return Some(id_username);
            }
        }

        None
    }
}

impl FromContext for UserCheckService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        let cookie = ctx.req.cookie();

        Ok(Self::new(
            ctx.inject().await?,
            cookie.get("login-token").map(|v| v.value_str().to_string()),
        ))
    }
}
