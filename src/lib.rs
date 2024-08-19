#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use futures_util::StreamExt;
use std::sync::Arc;
use std::thread;
use dashmap::DashMap;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

type Clients = Arc<DashMap<usize, tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>>;

#[napi]
pub struct WebSocketServer {
    clients: Clients,
}

#[napi]
impl WebSocketServer {
    #[napi(constructor, ts_args_type = "callback: (err: null | Error, result: number) => void")]
    pub fn new(port: u16, on_connect: JsFunction) -> Result<Self> {
        let clients = Arc::new(DashMap::new());
        let clients_clone = clients.clone();

        let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = on_connect.create_threadsafe_function(0, |ctx| {
          ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
        })?;

        tokio::spawn(async move {
            let addr = format!("0.0.0.0:{}", port);
            let listener = TcpListener::bind(&addr).await.unwrap();
            let mut id_counter = 0;

            while let Ok((stream, _)) = listener.accept().await {
                let ws_stream = accept_async(stream).await.unwrap();
                let id = id_counter;
                clients_clone.insert(id, ws_stream);
                id_counter += 1;

                //tsfn.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
            }
        });

        Ok(WebSocketServer { clients })
    }
}
