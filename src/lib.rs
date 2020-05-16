extern crate surf;

pub mod account;
// pub mod account_configurations;
// pub mod account_updates;
// pub mod account_activities;
// pub mod portfolio_history;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod market;
pub mod orders;
pub mod positions;
pub mod streaming;
pub mod utils;
pub mod watchlist;

pub use assets::*;
pub use calendar::*;
pub use clock::*;
pub use market::*;
pub use orders::*;
pub use positions::*;
pub use streaming::*;
pub use utils::*;
pub use watchlist::*;

use serde::{de::DeserializeOwned, Serialize};
use surf::http_types::Method;
use surf::middleware::HttpClient;
use surf::url::Url;
use surf::{Request, Response};

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

  pub(crate) async fn post<T: Serialize, U: DeserializeOwned>(
    &self,
    path: &str,
    data: &T,
    custom_subdomain: Option<&str>,
  ) -> Result<U, AlpakaError> {
    let url = self.url(custom_subdomain, path);
    let request = self.base_request(Method::Post, url).body_json(data)?;
    let mut response = request.await.unwrap();
    self.handle_response::<U>(&mut response).await
  }

  fn base_request(&self, method: Method, url: Url) -> Request<impl HttpClient> {
    let mut request = Request::new(method, url);
    request = request.set_header("APCA-API-KEY-ID".parse().unwrap(), &self.api_key);
    request = request.set_header("APCA-API-SECRET-KEY".parse().unwrap(), &self.api_secret);
    request
  }

  fn url(&self, custom_subdomain: Option<&str>, path: &str) -> Url {
    let subdomain = custom_subdomain
      .map(|x| x.to_string())
      .unwrap_or(self.mode.to_string());
    let url = format!("https:://{:?}.alpaca.markets/{}", subdomain, path);
    Url::parse(&url).unwrap()
  }

  async fn handle_response<U: DeserializeOwned>(
    &self,
    response: &mut Response,
  ) -> Result<U, AlpakaError> {
    let body = response.body_string().await;
    if response.status().is_success() {
      body
        .map_err(|err| AlpakaError::Unexpected(format!("{}", err)))
        .and_then(|payload| serde_json::from_str(&payload).map_err(AlpakaError::Json))
    } else {
      Err(AlpakaError::Unexpected(body.unwrap()))
    }
  }
}