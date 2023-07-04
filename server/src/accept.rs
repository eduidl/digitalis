use tokio_tungstenite::tungstenite::{handshake::server, Result};

const SEC_WEBSOCKET_PROTOCOL: &str = "Sec-WebSocket-Protocol";
const PROTOCOL: &str = "foxglove.websocket.v1";

#[derive(Debug, Clone, Copy)]
pub struct Accept;

impl server::Callback for Accept {
    fn on_request(
        self,
        request: &server::Request,
        response: server::Response,
    ) -> Result<server::Response, server::ErrorResponse> {
        if let Some(header) = request.headers().get(SEC_WEBSOCKET_PROTOCOL) {
            if header.to_str().unwrap().split(',').any(|v| v == PROTOCOL) {
                let mut response = response;
                response
                    .headers_mut()
                    .append(SEC_WEBSOCKET_PROTOCOL, PROTOCOL.parse().unwrap());

                log::info!("Accepting connection");
                return Ok(response);
            }
        }

        log::info!("Bad request: protocol mismatch");
        Err(server::Response::builder()
            .status(400)
            .body(Some("Bad Request".into()))
            .unwrap())
    }
}
