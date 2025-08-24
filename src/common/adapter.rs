use crate::common::error::{ErrorOutput, ErrorReportResponse, HtmlErrorOutput};
use error_stack::Report;
use poem::error::ResponseError;
use poem::{IntoResponse, Response};
use std::error::Error;
use std::marker::PhantomData;

pub struct ResultAdapter<T: IntoResponse, E: IntoResponse>(Result<T, E>);

impl<T: IntoResponse, E: IntoResponse> ResultAdapter<T, E> {
    pub async fn execute<FUT>(f: FUT) -> Self
    where
        FUT: Future<Output = Result<T, E>>,
    {
        ResultAdapter(f.await)
    }
}

impl<T: IntoResponse, E: IntoResponse> IntoResponse for ResultAdapter<T, E> {
    fn into_response(self) -> Response {
        match self.0 {
            Ok(t) => t.into_response(),
            Err(e) => e.into_response(),
        }
    }
}

pub struct ReportAdapter<T, E, O = HtmlErrorOutput>(Result<T, Report<E>>, PhantomData<O>)
where
    T: IntoResponse,
    E: ResponseError + Error + Send + Sync + 'static,
    O: ErrorOutput;

impl<T, E, O> ReportAdapter<T, E, O>
where
    T: IntoResponse,
    E: ResponseError + Error + Send + Sync + 'static,
    O: ErrorOutput,
{
    pub async fn execute<FUT>(f: FUT) -> Self
    where
        FUT: Future<Output = Result<T, Report<E>>>,
    {
        ReportAdapter(f.await, PhantomData)
    }
}

impl<T, E, O> IntoResponse for ReportAdapter<T, E, O>
where
    T: IntoResponse,
    E: ResponseError + Error + Send + Sync + 'static,
    O: ErrorOutput,
{
    fn into_response(self) -> Response {
        match self.0 {
            Ok(t) => t.into_response(),
            Err(e) => ErrorReportResponse::<E, O>::new(e).as_response(),
        }
    }
}
