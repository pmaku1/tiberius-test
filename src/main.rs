mod provider_model;
use provider_model::NYProviderFeed;
use serde_json::Value;
use tracing::log::info;
use std::error::Error;
use tiberius::{AuthMethod, Client, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use std::process;



#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {

    if let Err(err) = example().await {
        println!("error running example: {}", err);
        process::exit(1);
    }
    Ok(())
}


async fn example() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Password_01"));
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // To be able to use Tokio's tcp, we're using the `compat_write` from
    // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
    // traits from the `futures` crate.
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    // let mut rdr = csv::ReaderBuilder::new()
    //     .has_headers(true)
    //     .from_reader(io::stdin());

    let mut rdr = csv::Reader::from_path("/home/pmaku/projects/provider-test-data.csv")?;
    for result in rdr.deserialize(){
        let record: NYProviderFeed = result?;
        let id_insert = client.execute("SET  IDENTITY_INSERT provider.dbo.ProvMaster ON",&[] ).await;
        print!("{:#?}",id_insert);
        let mut query = Query::new("INSERT INTO provider.dbo.ProvMaster (ProvMasterKey) VALUES (@P1), (@P2), (@P3),(@p4), (@P5), (@P6),(@P7),(@P8), (@P9), (@P10), (@P11), (@P12), (@P13), (@P14),(@P15),(@P16),(@P18),(@P19)");
        query.bind(record.medicaid_provider_id);
        query.bind(record.npi);
        query.bind(record.provider_or_facility_name);
        query.bind(record.medicaid_type);
        query.bind(record.profession_or_service);
        query.bind(record.provider_specialty);
        query.bind(record.service_address);
        query.bind(record.city);
        query.bind(record.state);
        query.bind(record.zip_code);
        query.bind(record.county);
        query.bind(record.telephone);
        query.bind(record.latitude);
        query.bind(record.longitude);
        query.bind(record.enrollment_begin_date);
        query.bind(record.next_anticipated_revalidation_date);
        query.bind(record.file_date);
        query.bind(record.medically_fragile_children_directory_ind);
        query.bind(record.provider_email);
        let _results = query.execute(&mut client).await;

        println!("{:#?}",_results);
    }
    Ok(())
}


