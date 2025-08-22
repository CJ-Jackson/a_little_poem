use crate::common::html::HtmlBuilder;
use error_stack::{Context, Report, ResultExt};
use maud::{PreEscaped, html};
use poem::http::StatusCode;
use poem::web::Json;
use poem::{IntoResponse, Response};
use serde_json::json;
use std::error::Error;
use std::marker::PhantomData;
use thiserror::Error;

pub trait FromIntoStackError: Error + Sized + Send + Sync + 'static {
    fn from_error_stack<C>(err: &Report<C>) -> Option<&Self> {
        err.downcast_ref::<Self>()
    }

    fn is_in_error_stack<C>(err: &Report<C>) -> bool {
        Self::from_error_stack(err).is_some()
    }

    fn into_stack_error(self) -> Report<Self> {
        Report::new(self)
    }

    fn into_stack_error_critical(self, msg: String) -> Report<Self> {
        Report::new(self).attach(CriticalError(msg))
    }

    fn into_stack_error_as_attachment<E>(self, err: E) -> Report<E>
    where
        E: Error + Sized + Send + Sync + 'static,
    {
        Report::new(err).attach(self)
    }
}

#[derive(Error, Debug)]
#[error("Critical error: {0}")]
pub struct CriticalError(pub String);

impl FromIntoStackError for CriticalError {}

pub fn check_is_critical_error<C>(err: Report<C>) -> Result<Report<C>, Report<C>> {
    if CriticalError::is_in_error_stack::<C>(&err) {
        return Err(err);
    }
    Ok(err)
}

pub fn setup_critical_error_debug_hook() {
    Report::install_debug_hook::<CriticalError>(|value, context| {
        context.push_body(format!("Critical Error: {}", value.0))
    });
}

pub trait ExtraResultExt: ResultExt {
    fn attach_critical(self, msg: String) -> Result<Self::Ok, Report<Self::Context>>;

    fn attach_critical_lazy<F>(self, msg: F) -> Result<Self::Ok, Report<Self::Context>>
    where
        F: FnOnce() -> String;

    fn change_context_attach_previous_msg<C>(self, context: C) -> Result<Self::Ok, Report<C>>
    where
        C: Context;

    fn change_context_attach_previous_msg_lazy<C, F>(
        self,
        context: F,
    ) -> Result<Self::Ok, Report<C>>
    where
        C: Context,
        F: FnOnce() -> C;

    fn change_context_pass_ref_lazy<C, F>(self, context: F) -> Result<Self::Ok, Report<C>>
    where
        C: Context,
        F: FnOnce(&Report<Self::Context>) -> C;
}

impl<T, C> ExtraResultExt for error_stack::Result<T, C>
where
    C: Context,
{
    fn attach_critical(self, msg: String) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(report) => Err(report.attach(CriticalError(msg))),
        }
    }

    fn attach_critical_lazy<F>(self, msg: F) -> Self
    where
        F: FnOnce() -> String,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(report) => Err(report.attach(CriticalError(msg()))),
        }
    }

    fn change_context_attach_previous_msg<C2>(self, context: C2) -> Result<T, Report<C2>>
    where
        C2: Context,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(report) => {
                let msg = report.to_string();
                Err(report.change_context(context).attach_printable(msg))
            }
        }
    }

    fn change_context_attach_previous_msg_lazy<C2, F>(self, context: F) -> Result<T, Report<C2>>
    where
        C2: Context,
        F: FnOnce() -> C2,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(report) => {
                let msg = report.to_string();
                Err(report.change_context(context()).attach_printable(msg))
            }
        }
    }

    fn change_context_pass_ref_lazy<C2, F>(self, context: F) -> Result<T, Report<C2>>
    where
        C2: Context,
        F: FnOnce(&Report<Self::Context>) -> C2,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(report) => {
                let context = context(&report);
                Err(report.change_context(context))
            }
        }
    }
}

pub trait ErrorStatus: Error + Sized + Send + Sync + 'static {
    fn error_status(&self) -> StatusCode;
}

pub enum OutputType {
    Html,
    Json,
}

pub trait ErrorOutput: Send + Sync + 'static {
    const OUTPUT_TYPE: OutputType;

    fn output_type() -> OutputType {
        Self::OUTPUT_TYPE
    }
}

pub struct HtmlErrorOutput;

impl ErrorOutput for HtmlErrorOutput {
    const OUTPUT_TYPE: OutputType = OutputType::Html;
}

pub struct JsonErrorOutput;

impl ErrorOutput for JsonErrorOutput {
    const OUTPUT_TYPE: OutputType = OutputType::Json;
}

pub struct ErrorReportResponse<E, O = HtmlErrorOutput>(pub Report<E>, PhantomData<O>)
where
    E: ErrorStatus,
    O: ErrorOutput;

impl<E, O> ErrorReportResponse<E, O>
where
    E: ErrorStatus,
    O: ErrorOutput,
{
    pub fn new(report: Report<E>) -> Self {
        Self(report, PhantomData)
    }
}

impl<E, O> IntoResponse for ErrorReportResponse<E, O>
where
    E: ErrorStatus,
    O: ErrorOutput,
{
    fn into_response(self) -> Response {
        let status = self.0.current_context().error_status();
        let pre = if cfg!(debug_assertions) {
            format!("{:?}", self.0)
        } else {
            format!("{}", self.0)
        };

        let title = format!("Error: {}", status.to_string());

        match O::output_type() {
            OutputType::Html => {
                let html = HtmlBuilder::new(
                    title.clone(),
                    html! {
                        div .container .main-content .mt-3 .px-7 .py-7 .mx-auto {
                            h1 .mt-3 { (title.to_string()) }
                            pre .mt-3 { (PreEscaped(pre)) }
                        }
                    },
                )
                .build();

                html.with_status(status).into_response()
            }
            OutputType::Json => {
                let json = Json(json!({
                    "title": title,
                    "pre": pre
                }));

                json.with_status(status).into_response()
            }
        }
    }
}
