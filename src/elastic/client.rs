use url::Url;

use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch, Error,
};

use super::super::settings;

pub fn get_elastic_client(connection: &settings::Connection) -> Result<Elasticsearch, Error> {
    let url = Url::parse(&connection.get_url());
    let url = match url {
        Ok(url) => url,
        Err(error) => {
            eprintln!("Failed to parse url \"{}\": {:?}", &connection.get_url(), error);
            std::process::exit(exitcode::CONFIG);
        }
    };
    let conn_pool = SingleNodeConnectionPool::new(url);
    let mut builder = TransportBuilder::new(conn_pool).disable_proxy();
    if connection.has_auth() {
        let credentials = Credentials::Basic(
            connection.username.as_ref().unwrap().to_string(),
            connection.password.as_ref().unwrap().to_string(),
        );
        builder = builder.auth(credentials);
    }
    return Ok(Elasticsearch::new(builder.build()?));
}
