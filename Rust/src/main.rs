use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello.v1");
}

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};

#[derive(Default)]
struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let message = if name.trim().is_empty() {
            "Hello from Rust gRPC!".to_string()
        } else {
            format!("Hello, {name}! From Rust gRPC.")
        };

        Ok(Response::new(HelloReply { message }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter;

    println!("gRPC server listening on {addr}");

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hello::greeter_client::GreeterClient;
    use tokio::sync::oneshot;
    use tokio_stream::wrappers::TcpListenerStream;

    #[tokio::test]
    async fn say_hello_rpc_is_available_to_consumers() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind test listener");
        let addr = listener.local_addr().expect("read local addr");
        let incoming = TcpListenerStream::new(listener);
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(GreeterServer::new(MyGreeter))
                .serve_with_incoming_shutdown(incoming, async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("run grpc server");
        });

        let endpoint = format!("http://{addr}");
        let mut client = GreeterClient::connect(endpoint)
            .await
            .expect("connect generated client");

        let response = client
            .say_hello(Request::new(HelloRequest {
                name: "consumer".to_string(),
            }))
            .await
            .expect("call SayHello RPC");

        assert_eq!(
            response.into_inner().message,
            "Hello, consumer! From Rust gRPC."
        );

        let _ = shutdown_tx.send(());
        server.await.expect("join server task");
    }
}
