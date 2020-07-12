![image](https://user-images.githubusercontent.com/4346014/87255815-4b8aa900-c48e-11ea-9dc6-06f5139921ed.png)

`alpaka` is a crate (i.e., Rust library) that allows you to trade through the [Alpaca](alpaca.markets) API-broker. The library is powered by [surf]() & compatible with [async-std]()'s crates suit.

This client implements the vast majority of Alpaca's API endpoints, including:

Retrieve assets information & their last quote.
Open, close & retrieve positions.
Retrieve market's clock & calendar

#### Usage

> _**NOTE**: There is a bunch of runnable [examples](https://github.com/tehAnswer/alpaka/tree/master/examples)_

First, you need to create an instance of `alpaka::Alpaka` client by providing API keys and the trading mode: Paper (staging) or Live (production).

```rust
use alpaka::*;

let api_key: String = std::env::var("API_KEY").unwrap();
let secret_key: String = std::env::var("SECRET_KEY").unwrap();
let client = Alpaka::new(api_key, secret_key, AlpakaMode::Paper);
```

Then you can use multiple services to perform operations on a given Alpaca API resource. At the moment, the library exposes the following ones:

- **accounts**: returns information about the current user. 
- **assets**: returns information (excluding price) about different stock symbols.
- **calendars**: returns information about the future market sessions.
- **clock**: tells if the market is open and its next opening and closing.
- **orders**: allows users to manage their orders.
- **positions**: enables users to manage their stock positions.
- **quotes**: returns the last stock price (bid & ask) for a given ticker symbol.
- **watchlists**: allows users to manage watchlists of stocks.

For that, it is as simple as calling their appropriate gateway method to create a proxy service. Proxy services will expose different sets of methods depending on the operations that can be performed over those API resources. 

```rust
let positions = client.positions();
println!("{:?}", positions.close_all());
```


