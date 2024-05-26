//! # Monitor core API
//!
//! Monitor core exposes an HTTP api using standard JSON serialization.
//!
//! All calls share some common HTTP params:
//! - Method: `POST`
//! - Path: `/auth`, `/user`, `/read`, `/write`, `/execute`
//! - Headers:
//!   - Content-Type: `application/json`
//!   - Authorication: `your_jwt`
//!   - X-Api-Key: `your_api_key`
//!   - X-Api-Secret: `your_api_secret`
//!   - Use either Authorization *or* X-Api-Key and X-Api-Secret to authenticate requests.
//! - Body: JSON specifying the request type (`type`) and the parameters (`params`).
//! The request type matches the name of the the request struct definition,
//! and the params match the fields of the request struct.
//!
//! For example, this is an example body for [read::GetDeployment]:
//! ```
//! {
//!   "type": "GetDeployment",
//!   "params": {
//!     "deployment": "66113df3abe32960b87018dd"
//!   }
//! }
//! ```
//!
//! The request's parent module (eg. [read], [mod@write]) determines the http path which
//! must be used for the requests. For example, requests under [read] are made using http path `/read`.
//!
//! ## Modules
//!
//! - [auth]: Requests relating to logging in / obtaining authentication tokens.
//! - [user]: User self-management actions (manage api keys, etc.)
//! - [read]: Read only requests which retrieve data from Monitor.
//! - [execute]: Run actions on monitor resources, eg [execute::RunBuild].
//! - [mod@write]: Requests which alter data, like create / update / delete resources.
//!
//! ## Errors
//!
//! Request errors will be returned with a JSON body containing information about the error.
//! They will have the following common format:
//! ```
//! {
//!   "error": "top level error message",
//!   "trace": [
//!     "first traceback message",
//!     "second traceback message"
//!   ]
//! }
//! ```

pub mod auth;
pub mod execute;
pub mod read;
pub mod user;
pub mod write;
