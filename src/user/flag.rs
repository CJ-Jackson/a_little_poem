use crate::common::context::user::UserContextDependencyFlag;

pub struct LoginFlag;

impl UserContextDependencyFlag for LoginFlag {
    const ALLOW_USER: bool = false;
    const ALLOW_VISITOR: bool = true;
}

pub struct LogoutFlag;

impl UserContextDependencyFlag for LogoutFlag {
    const ALLOW_USER: bool = true;
    const ALLOW_VISITOR: bool = false;
}
