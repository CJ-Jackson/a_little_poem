use poem::http::StatusCode;
use poem::{FromRequest, IntoResponse, Request, RequestBody, Response};

pub struct EtagStamp {
    pub data: Box<[u8]>,
    pub content_type: &'static str,
}

impl IntoResponse for EtagStamp {
    fn into_response(self) -> Response {
        let data = self.data.to_vec();
        match option_env!("ETAG") {
            None => data.with_header("X-ETag", "not-set"),
            Some(etag) => data.with_header("ETag", etag),
        }
        .with_header("Content-Type", self.content_type)
        .into_response()
    }
}

pub struct EtagCheck;

impl<'a> FromRequest<'a> for EtagCheck {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let etag = match option_env!("ETAG") {
            None => return Ok(Self),
            Some(etag) => etag,
        };

        let etag_header = req.header("If-None-Match");
        match etag_header {
            None => Ok(Self),
            Some(etag_header) if etag_header == etag => {
                return Err(poem::Error::from_status(StatusCode::NOT_MODIFIED));
            }
            Some(_) => Ok(Self),
        }
    }
}
