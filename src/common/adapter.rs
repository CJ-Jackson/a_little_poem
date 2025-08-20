use poem::{IntoResponse, Response};

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
