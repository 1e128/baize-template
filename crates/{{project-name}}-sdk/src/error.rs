use baizekit::api::ErrorCode;
use sea_orm::DbErr;
pub use snafu::ResultExt;
use snafu::{IntoError, Location, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}"))]
    DbError {
        source: DbErr,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("{}", message))]
    InvalidParams {
        message: String,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("{}", message))]
    NotFound {
        message: String,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("InternalServerError: {}", message))]
    InternalServer {
        message: String,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("unknown error {}", message))]
    NotImplemented {
        message: String,
        #[snafu(implicit)]
        location: Location,
    },
}

impl From<DbErr> for Error {
    fn from(value: DbErr) -> Self {
        DbSnafu.into_error(value)
    }
}

impl ErrorCode for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
pub type ApiResult<T, E = Error> = baizekit::api::ApiResult<T, E>;
