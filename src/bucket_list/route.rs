use crate::bucket_list::model::{
    AddToBucketList, AddToBucketListResult, AddToBucketListValidationErrorResponse, BucketListItem,
};
use crate::bucket_list::repository::{BucketListRepository, BucketListRepositoryError};
use crate::common::adapter::{ReportAdapter, ResultAdapter};
use crate::common::context::Dep;
use crate::common::context::user::JustDep;
use crate::common::csrf::CsrfHeaderChecker;
use crate::common::error::{ErrorReportResponse, JsonErrorOutput};
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::icon::plus_icon;
use maud::{Markup, PreEscaped, html};
use poem::http::StatusCode;
use poem::i18n::Locale;
use poem::web::{Json, WithStatus};
use poem::{IntoResponse, Response, Route, get, handler, post};
use serde_json::{Value, json};

#[handler]
async fn main_bucket_list(JustDep(context_html_builder, _): JustDep<ContextHtmlBuilder>) -> Markup {
    let title = "Bucket List";
    context_html_builder
        .attach_title(title)
        .set_current_tag("bucket-list")
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

#[handler]
async fn all_bucket_list(
    Dep(repo): Dep<BucketListRepository>,
) -> ReportAdapter<Json<Box<[BucketListItem]>>, BucketListRepositoryError, JsonErrorOutput> {
    ReportAdapter::execute(async {
        let items = repo.get_all_from_bucket_list()?;
        Ok(Json(items))
    })
    .await
}

enum AddBucketListRouteError {
    Repo(ErrorReportResponse<BucketListRepositoryError, JsonErrorOutput>),
    Validate(Json<AddToBucketListValidationErrorResponse>),
}

impl IntoResponse for AddBucketListRouteError {
    fn into_response(self) -> Response {
        match self {
            Self::Repo(err) => err.into_response(),
            Self::Validate(err) => err
                .with_status(StatusCode::UNPROCESSABLE_ENTITY)
                .into_response(),
        }
    }
}

#[handler]
async fn add_bucket_list(
    Dep(repo): Dep<BucketListRepository>,
    Json(data): Json<AddToBucketList>,
    _csrf_header_checker: CsrfHeaderChecker,
    locale: Locale,
) -> ResultAdapter<WithStatus<Json<Value>>, AddBucketListRouteError> {
    ResultAdapter::execute(async {
        let AddToBucketListResult(data) = (&data).into();
        let data =
            data.map_err(|e| AddBucketListRouteError::Validate(Json((e, &locale).into())))?;

        repo.add_to_bucket_list(&data)
            .map_err(|e| AddBucketListRouteError::Repo(ErrorReportResponse::new(e)))?;

        Ok(Json(json!({"message": "Success"})).with_status(StatusCode::CREATED))
    })
    .await
}

pub fn route_bucket_list() -> Route {
    Route::new()
        .at("/", get(main_bucket_list))
        .at("/all", get(all_bucket_list))
        .at("/add", post(add_bucket_list))
}
