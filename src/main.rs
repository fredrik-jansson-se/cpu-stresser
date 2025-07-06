use clap::Parser;

pub mod load {
    tonic::include_proto!("load");
}

#[derive(Default)]
struct MyLoadService;

#[tonic::async_trait]
impl load::load_service_server::LoadService for MyLoadService {
    async fn set_load(
        &self,
        _request: tonic::Request<load::Load>,
    ) -> Result<tonic::Response<load::Empty>, tonic::Status> {
        Ok(tonic::Response::new(load::Empty {}))
    }
}

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Server,
    Client
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    match opts.command {
        Command::Server => {
            let addr = "[::1]:50051".parse()?;
            let service = MyLoadService::default();
            let server = load::load_service_server::LoadServiceServer::new(service);
            tonic::transport::Server::builder()
                .add_service(server)
                .serve(addr)
                .await?;
        }
        Command::Client => {
            let mut client =
                load::load_service_client::LoadServiceClient::connect("http://[::1]:50051")
                    .await?;
            let request = tonic::Request::new(load::Load {
                cpus: 1,
                time_seconds: 1,
            });
            let response = client.set_load(request).await?;
            println!("Response: {:?}", response.into_inner());
        }
    }
    Ok(())
}
