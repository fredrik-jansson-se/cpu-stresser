use clap::Parser;

pub mod load {
    tonic::include_proto!("load");
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
            let server = load::load_server::LoadServer::default();
            tonic::transport::Server::builder()
                .add_service(server)
                .serve(addr)
                .await?;
        }
        Command::Client => {
            let mut client = load::load_client::LoadClient::connect("http://[::1]:50051").await?;
            let request = tonic::Request::new(load::LoadRequest {});
            let response = client.load(request).await?;
            println!("Response: {:?}", response.into_inner());
        }
    }
    Ok(())
}
