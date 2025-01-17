mod parser;
mod transformer;

use istio_api_rs::networking::v1beta1::destination_rule::*;
use istio_api_rs::networking::v1beta1::gateway::*;
use istio_api_rs::networking::v1beta1::virtual_service::*;
use kube::{
    api::{Api, ListParams, ResourceExt},
    client::ConfigExt,
    config::{KubeConfigOptions, Kubeconfig},
    Client, Config,
};
use tower::ServiceBuilder;

const NAMESPACE: &str = "default";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let kube_config_file = std::fs::File::open("/home/shane/kube/config.yaml")?;
    let kube_config: Kubeconfig = serde_yaml::from_reader(kube_config_file)?;
    let kube_config_opt = KubeConfigOptions::default();
    let kube_config = Config::from_custom_kubeconfig(kube_config, &kube_config_opt).await?;
    let https = kube_config.openssl_https_connector()?;

    tracing_subscriber::fmt::init();

    let service = ServiceBuilder::new()
        .layer(kube_config.base_uri_layer())
        .option_layer(kube_config.auth_layer()?)
        .service(hyper::Client::builder().build(https));
    let client = Client::new(service, kube_config.default_namespace);
    let list_opt = ListParams::default();

    let gws: Api<Gateway> = Api::namespaced(client.clone(), NAMESPACE);
    for gw in gws.list(&list_opt).await? {
        println!("Found Gateway: {}", gw.name());
    }

    let drs: Api<DestinationRule> = Api::namespaced(client.clone(), NAMESPACE);
    for dr in drs.list(&list_opt).await? {
        println!("Found Destination Rule: {}", dr.name());
    }

    let vss: Api<VirtualService> = Api::namespaced(client.clone(), NAMESPACE);
    for vs in vss.list(&list_opt).await? {
        let content = serde_yaml::to_string(&vs).unwrap();
        println!("Found Virtual Service with YAML content: {}", content);
    }

    // Read DSL from a file
    let dsl = fs::read_to_string("example.dsl").expect("Unable to read file");

    // Parse the DSL
    let (_, statements) = dsl_parser::parse_dsl(&dsl).expect("Failed to parse DSL");

    // Transform the parsed DSL into OAM and Istio manifests
    // dsl_transformer::transform(statements



    Ok(())
}

fn main() {}