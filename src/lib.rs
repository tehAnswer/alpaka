extern crate chrono;
extern crate http_types;
extern crate mockito;
extern crate surf;

pub mod accounts;
// pub mod account_configurations;
// pub mod account_updates;
// pub mod account_activities;
// pub mod portfolio_history;
pub mod assets;
pub mod calendars;
pub mod clock;
pub mod orders;
pub mod positions;
pub mod streaming;
pub mod utils;
pub mod watchlist;

pub use accounts::*;
pub use assets::*;
pub use calendars::*;
pub use clock::*;
pub use orders::*;
pub use positions::*;
pub use streaming::*;
pub use utils::*;
pub use watchlist::*;

use http_types::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use surf::http_types::Method;
use surf::middleware::HttpClient;
use surf::url::Url;
use surf::Request;

pub struct Alpaka {
  api_key: String,
  api_secret: String,
  mode: AlpakaMode,
}

impl Alpaka {
  pub fn new(api_key: String, api_secret: String, mode: AlpakaMode) -> Alpaka {
    Alpaka {
      api_key,
      api_secret,
      mode,
    }
  }

  pub fn orders(&self) -> Orders {
    Orders::new(Box::new(&self))
  }

  pub fn accounts(&self) -> Accounts {
    Accounts::new(Box::new(&self))
  }

  pub fn assets(&self) -> Assets {
    Assets::new(Box::new(&self))
  }

  pub fn positions(&self) -> Positions {
    Positions::new(Box::new(&self))
  }

  pub fn watchlists(&self) -> Watchlists {
    Watchlists::new(Box::new(&self))
  }

  pub fn calendars(&self) -> Calendars {
    Calendars::new(Box::new(&self))
  }

  pub(crate) async fn post<
    T: Serialize + std::fmt::Debug,
    U: DeserializeOwned + std::fmt::Debug + std::default::Default,
  >(
    &self,
    path: &str,
    data: &T,
    custom_subdomain: Option<&str>,
  ) -> Result<U, AlpakaError> {
    let url = self.url(custom_subdomain, path);
    let request = self.base_request(Method::Post, url).body_json(data)?;
    self.execute_request(request).await
  }

  pub(crate) async fn get<
    T: Serialize + std::fmt::Debug,
    U: DeserializeOwned + std::fmt::Debug + std::default::Default,
  >(
    &self,
    path: &str,
    data: &T,
    custom_subdomain: Option<&str>,
  ) -> Result<U, AlpakaError> {
    let url = self.url(custom_subdomain, path);
    let request = self
      .base_request(Method::Get, url)
      .set_query(&data)
      .map_err(AlpakaError::UrlEncodeError)?;
    self.execute_request(request).await
  }

  pub(crate) async fn delete<
    T: Serialize + std::fmt::Debug,
    U: DeserializeOwned + std::fmt::Debug + std::default::Default,
  >(
    &self,
    path: &str,
    data: &T,
    custom_subdomain: Option<&str>,
  ) -> Result<U, AlpakaError> {
    let url = self.url(custom_subdomain, path);
    let request = self
      .base_request(Method::Delete, url)
      .set_query(&data)
      .map_err(AlpakaError::UrlEncodeError)?;
    self.execute_request(request).await
  }

  pub(crate) async fn put<
    T: Serialize + std::fmt::Debug,
    U: DeserializeOwned + std::fmt::Debug + std::default::Default,
  >(
    &self,
    path: &str,
    data: &T,
    custom_subdomain: Option<&str>,
  ) -> Result<U, AlpakaError> {
    let url = self.url(custom_subdomain, path);
    let request = self.base_request(Method::Put, url).body_json(data)?;
    self.execute_request(request).await
  }

  async fn execute_request<U: DeserializeOwned + std::fmt::Debug + std::default::Default>(
    &self,
    request: Request<impl HttpClient>,
  ) -> Result<U, AlpakaError> {
    let mut response: surf::Response = request.await.map_err(AlpakaError::RequestError)?;

    if response.status() == StatusCode::NoContent {
      Ok(U::default())
    } else if response.status().is_success() {
      response.body_json().await.map_err(AlpakaError::IOError)
    } else {
      let error_details: APIErrorDetails =
        response.body_json().await.map_err(AlpakaError::IOError)?;
      let error = AlpakaError::APIError(error_details.code, error_details.message);
      Err(error)
    }
  }

  fn base_request(&self, method: Method, url: Url) -> Request<impl HttpClient> {
    let mut request = Request::new(method, url);
    request = request.set_header("APCA-API-KEY-ID", self.api_key.to_owned());
    request = request.set_header("APCA-API-SECRET-KEY", self.api_secret.to_owned());
    request
  }

  #[cfg(not(test))]
  fn url(&self, custom_subdomain: Option<&str>, path: &str) -> Url {
    let subdomain = custom_subdomain
      .map(|x| x.to_string())
      .unwrap_or_else(|| self.mode.to_string());
    let base_url: &str = "alpaca.markets";
    let url = format!("https://{}.{}/{}", subdomain, base_url, path);
    Url::parse(&url).unwrap()
  }

  #[cfg(test)]
  fn url(&self, _: Option<&str>, path: &str) -> Url {
    let base_url = mockito::server_url();
    let url = format!("{}/{}", base_url, path);
    Url::parse(&url).unwrap()
  }
}
