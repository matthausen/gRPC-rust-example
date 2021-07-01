use tonic::{transport:: Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayResponse, SayRequest};
pub mod hello {
    tonic::include_proto!("hello"); // The string specified here must match the proto package name
}


#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request:Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        
        let reply = hello::SayResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let say = MySay::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}