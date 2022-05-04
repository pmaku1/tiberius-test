mod provider_model;
use provider_model::NYProviderFeed;
use std::error::Error;
use std::process;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tracing::{error, info, log::trace};
use tracing_subscriber;

// static INPUT_FILE: &str = "/Users/patrickmaku/Projects/tiberius-test/provider-test-data.csv";
static INPUT_FILE: &str = "./provider-test-data.csv";


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // tracing_subscriber::fmt::init()
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("application start");

    if let Err(err) = example().await {
        error!("error running example: {}", err);
        process::exit(1);
    }
    Ok(())
}

async fn example() -> Result<(), Box<dyn Error>> {
    trace!("setup database connection ...");

    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.database("provider");
    config.authentication(AuthMethod::sql_server("sa", "Password_01"));

    if cfg!(target_os = "macos") {
        // Hack to get past TLS connection issue on macOS v12.3.1
        config.encryption(tiberius::EncryptionLevel::NotSupported);
    }
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;
    let mut rdr = csv::Reader::from_path(INPUT_FILE)?;

    info!("load database ...");
    for result in rdr.deserialize() {
        let record: NYProviderFeed = result?;
        let payload = format!("INSERT into dbo.ProvMaster VALUES ({},{},'{}','{}','{}','{}','{}','{}','{}','{}','{}','{}',{},{},'{}','{}','{}','{}','{}')",
                &record.medicaid_provider_id.unwrap_or(0), 
                &record.npi.unwrap_or(0),
                &record.provider_or_facility_name.unwrap_or("".to_string()),
                &record.medicaid_type.unwrap_or("".to_string()),
                &record.profession_or_service.unwrap_or("".to_string()),
                &record.provider_specialty.unwrap_or("".to_string()),
                &record.service_address.unwrap_or("".to_string()),
                &record.city.unwrap_or("".to_string()),
                &record.state.unwrap_or("".to_string()),
                &record.zip_code.unwrap_or("".to_string()),
                &record.county.unwrap_or("".to_string()),
                &record.telephone.unwrap_or("".to_string()),
                &record.latitude.unwrap_or(0.0),
                &record.longitude.unwrap_or(0.0),
                &record.enrollment_begin_date.unwrap(),
                &record.next_anticipated_revalidation_date.unwrap(),
                &record.file_date.unwrap(),
                &record.medically_fragile_children_directory_ind.unwrap_or("".to_string()),
                &record.provider_email.unwrap_or("".to_string()),
            );
        let _result = client.simple_query(payload).await;

    }
    info!("database load complete ...");

    Ok(())
}

