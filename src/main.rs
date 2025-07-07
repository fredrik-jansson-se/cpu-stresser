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
        request: tonic::Request<load::Load>,
    ) -> Result<tonic::Response<load::Empty>, tonic::Status> {
        tracing::info!("Received load request: {:?}", request);

        let load::Load {
            cpus,
            time_seconds,
        } = request.into_inner();

        let cpus = cpus.max(1) as usize;
        let duration = std::time::Duration::from_secs(time_seconds.max(1) as u64);

        for _ in 0..cpus {
            tokio::task::spawn_blocking({
                move || {
                    let end = std::time::Instant::now() + duration;
                    while std::time::Instant::now() < end {
                        std::hint::spin_loop();
                    }
                }
            });
        }

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
    Client {
        server_address: String,
        num_cpus: Option<i32>,
        time_seconds: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    match opts.command {
        Command::Server => {
            tracing::info!("Starting server on [::1]:20051");
            let addr = "0.0.0.0:20051".parse()?;
            let service = MyLoadService{};
            let server = load::load_service_server::LoadServiceServer::new(service);
            tonic::transport::Server::builder()
                .add_service(server)
                .serve(addr)
                .await?;
        }
        Command::Client {
            server_address,
            num_cpus,
            time_seconds,
        } => {
            let mut client =
                load::load_service_client::LoadServiceClient::connect(format!("http://{server_address}:20051")).await?;
            let request = tonic::Request::new(load::Load {
                cpus: num_cpus.unwrap_or(1),
                time_seconds: time_seconds.unwrap_or(5),
            });
            let response = client.set_load(request).await?;
            println!("Response: {:?}", response.into_inner());
        }
    }
    Ok(())
}
