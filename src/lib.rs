extern crate chrono;
extern crate mockito;
extern crate surf;

pub mod account;
// pub mod account_configurations;
// pub mod account_updates;
// pub mod account_activities;
// pub mod portfolio_history;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod orders;
pub mod positions;
pub mod streaming;
pub mod utils;
pub mod watchlist;

pub use assets::*;
pub use calendar::*;
pub use clock::*;
pub use orders::*;
pub use positions::*;
pub use streaming::*;
pub use utils::*;
pub use watchlist::*;

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
    let result: Result<U, surf::Error> = request.recv_json().await;
    result.map_err(AlpakaError::RequestError)
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
    let result: Result<U, surf::Error> = request.recv_json().await;
    result.map_err(AlpakaError::RequestError)
  }

  fn base_request(&self, method: Method, url: Url) -> Request<impl HttpClient> {
    let mut request = Request::new(method, url);
    request = request.set_header("APCA-API-KEY-ID".parse().unwrap(), &self.api_key);
    request = request.set_header("APCA-API-SECRET-KEY".parse().unwrap(), &self.api_secret);
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
