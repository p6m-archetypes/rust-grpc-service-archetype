mod management;
mod readiness;
pub mod settings;

use anyhow::Result;
use readiness::ReadinessState;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

use {{ prefix_name }}_{{ suffix_name }}_core::{
    proto::{{ prefix_name }}_{{ suffix_name }}_server::{{ PrefixName }}{{ SuffixName }}Server as {{ PrefixName }}{{ SuffixName }}ProtoServer, {{ PrefixName }}{{ SuffixName }}Core,
};

pub use settings::ServerSettings;

pub struct {{ PrefixName }}{{ SuffixName }}Server {
    core: {{ PrefixName }}{{ SuffixName }}Core,
    grpc_listener: Option<TcpListener>,
    management_listener: Option<TcpListener>,
    grpc_port: u16,
    management_port: u16,
}

pub struct Builder {
    core: {{ PrefixName }}{{ SuffixName }}Core,
    settings: ServerSettings,
}

impl Builder {
    pub fn new(core: {{ PrefixName }}{{ SuffixName }}Core) -> Self {
        Self {
            core,
            settings: ServerSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: &ServerSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Server> {
        let grpc_addr = format!("{}:{}", self.settings.host, self.settings.port);
        let mgmt_addr = format!("{}:{}", self.settings.host, self.settings.management_port);

        let grpc_listener = TcpListener::bind(&grpc_addr).await?;
        let grpc_port = grpc_listener.local_addr()?.port();

        let management_listener = TcpListener::bind(&mgmt_addr).await?;
        let management_port = management_listener.local_addr()?.port();

        Ok({{ PrefixName }}{{ SuffixName }}Server {
            core: self.core,
            grpc_listener: Some(grpc_listener),
            management_listener: Some(management_listener),
            grpc_port,
            management_port,
        })
    }
}

impl {{ PrefixName }}{{ SuffixName }}Server {
    pub fn builder(core: {{ PrefixName }}{{ SuffixName }}Core) -> Builder {
        Builder::new(core)
    }

    pub fn grpc_port(&self) -> u16 {
        self.grpc_port
    }

    pub fn management_port(&self) -> u16 {
        self.management_port
    }

    pub async fn serve(mut self) -> Result<()> {
        let grpc_listener = self.grpc_listener.take().expect("listener already consumed");
        let management_listener = self.management_listener.take().expect("listener already consumed");

        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<{{ PrefixName }}{{ SuffixName }}ProtoServer<{{ PrefixName }}{{ SuffixName }}Core>>()
            .await;

        let reflection = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set({{ prefix_name }}_{{ suffix_name }}_core::proto::FILE_DESCRIPTOR_SET)
            .build_v1()
            .unwrap();

        let grpc = Server::builder()
            .add_service(health_service)
            .add_service(reflection)
            .add_service({{ PrefixName }}{{ SuffixName }}ProtoServer::new(self.core));

        tracing::info!("gRPC listening on port {}", self.grpc_port);

        let readiness = ReadinessState::new();
        let readiness_for_grpc = readiness.clone();

        let _ = tokio::join!(
            async move {
                readiness_for_grpc.set_grpc_ready(true).await;
                grpc.serve_with_incoming(TcpListenerStream::new(grpc_listener))
                    .await
                    .unwrap();
            },
            management::serve(management_listener, readiness),
        );

        Ok(())
    }
}
