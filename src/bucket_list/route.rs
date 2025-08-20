use crate::bucket_list::model::{AddToBucketList, BucketListItem};
use crate::bucket_list::repository::{BucketListRepository, BucketListRepositoryError};
use crate::common::context::Dep;
use crate::common::error::{ErrorReportResponse, JsonErrorOutput};
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::icon::plus_icon;
use crate::common::validation::ValidationErrorResponse;
use error_stack::Report;
use maud::{Markup, PreEscaped, html};
use poem::http::StatusCode;
use poem::web::Json;
use poem::{IntoResponse, Response, Route, get, handler, post};
use serde_json::{Value, json};

const PREFIX: &str = "/bucket-list";

#[handler]
async fn main_bucket_list(context_html_builder: Dep<ContextHtmlBuilder>) -> Markup {
    let title = "Bucket List";
    context_html_builder
        .0
        .attach_title(title)
        .attach_content(html! {
            h1 .mt-3 { (title) }
            div #bucket-list .mt-3 v-cloak {
                div .bucket-list-header {
                    span .bucket-list-col { "ID" }
                    span .bucket-list-col { "Name" }
                    span .bucket-list-col { "Description" }
                    span .bucket-list-col { "Timestamp" }
                }
                div .bucket-list-item "v-for"="item in bucket_list" {
                    span .bucket-list-col { "{{ item.id }}" }
                    span .bucket-list-col { "{{ item.name }}" }
                    span .bucket-list-col { "{{ item.description }}" }
                    span .bucket-list-col { "{{ item.timestamp }}" }
                }
                div .bucket-form .mt-5 {
                    input .bucket-list-col .bucket-form-input
                        type="text" placeholder="Name" "v-model"="input_name";
                    input .bucket-list-col .bucket-form-input
                        type="text" placeholder="Description" "v-model"="input_description";
                    button .bucket-list-col .btn .btn-sky-blue "v-on:click"="addToBucketList" {
                        "Add"
                        (plus_icon())
                    }
                }
                div .bucket-form-error "v-if"="error" {
                    span .bucket-list-col {
                        ul {
                            li "v-for"="message in error.name" { "{{ message }}" }
                        }
                    }
                    span .bucket-list-col {
                        ul {
                            li "v-for"="message in error.description" { "{{ message }}" }
                        }
                    }
                    span .bucket-list-col {}
                }
            }
        })
        .attach_footer(get_bucket_list_js())
        .build()
}

fn get_bucket_list_js() -> Markup {
    #[cfg(debug_assertions)]
    let js = include_str!("_asset/bucket_list.js");
    #[cfg(not(debug_assertions))]
    let js = include_str!("_asset/bucket_list.min.js");
    html! {
        script type="module" { (PreEscaped(js)) }
    }
}

struct AllBucketListResult(
    Result<
        Json<Box<[BucketListItem]>>,
        ErrorReportResponse<BucketListRepositoryError, JsonErrorOutput>,
    >,
);

impl AllBucketListResult {
    async fn execute<FUT>(f: FUT) -> Self
    where
        FUT: Future<Output = Result<Box<[BucketListItem]>, Report<BucketListRepositoryError>>>,
    {
        match f.await {
            Ok(items) => Self(Ok(Json(items))),
            Err(err) => Self(Err(ErrorReportResponse::new(err))),
        }
    }
}

impl IntoResponse for AllBucketListResult {
    fn into_response(self) -> Response {
        match self.0 {
            Ok(json) => json.into_response(),
            Err(err) => err.into_response(),
        }
    }
}

#[handler]
async fn all_bucket_list(repo: Dep<BucketListRepository>) -> AllBucketListResult {
    AllBucketListResult::execute(async {
        let items = repo.0.get_all_from_bucket_list()?;
        Ok(items)
    })
    .await
}

enum AddBucketListRouteError {
    Repo(ErrorReportResponse<BucketListRepositoryError, JsonErrorOutput>),
    Validate(ValidationErrorResponse),
}

impl IntoResponse for AddBucketListRouteError {
    fn into_response(self) -> Response {
        match self {
            Self::Repo(err) => err.into_response(),
            Self::Validate(err) => err.into_response(),
        }
    }
}

struct AddBucketListRouteResult(Result<Value, AddBucketListRouteError>);

impl AddBucketListRouteResult {
    async fn execute<FUT>(f: FUT) -> Self
    where
        FUT: Future<Output = Result<Value, AddBucketListRouteError>>,
    {
        match f.await {
            Ok(value) => Self(Ok(value)),
            Err(err) => Self(Err(err)),
        }
    }
}

impl IntoResponse for AddBucketListRouteResult {
    fn into_response(self) -> Response {
        match self.0 {
            Ok(value) => value
                .to_string()
                .with_status(StatusCode::CREATED)
                .with_content_type("application/json")
                .into_response(),
            Err(err) => err.into_response(),
        }
    }
}

#[handler]
async fn add_bucket_list(
    repo: Dep<BucketListRepository>,
    data: Json<AddToBucketList>,
) -> AddBucketListRouteResult {
    AddBucketListRouteResult::execute(async {
        let data = data
            .to_validated()
            .map_err(|e| AddBucketListRouteError::Validate(e))?;

        repo.0
            .add_to_bucket_list(&data)
            .map_err(|e| AddBucketListRouteError::Repo(ErrorReportResponse::new(e)))?;

        Ok(json!({"message": "Success"}))
    })
    .await
}

pub fn route_bucket_list(route: Route) -> Route {
    route
        .at(format!("{PREFIX}/"), get(main_bucket_list))
        .at(format!("{PREFIX}/all"), get(all_bucket_list))
        .at(format!("{PREFIX}/add"), post(add_bucket_list))
}
