use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_ping))
        .route("/ws", get(ws_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn get_ping() -> &'static str {
    "pong!"
}

#[cfg(test)]
mod tests {
    use axum::{routing::get, Router};
    use axum_test::TestServer;

    use crate::get_ping;

    #[tokio::test]
    async fn it_should_get() {
        let ping = "/ping";
        // Build an application with a route.
        let app = Router::new().route(ping, get(get_ping));
        // Run the application for testing.
        let server = TestServer::new(app).unwrap();
        // Get the request.
        let response = server.get(ping).await;
        assert_eq!(response.text(), "pong!");
    }
}
