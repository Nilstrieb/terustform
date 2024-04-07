use std::{
    env,
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use eyre::{bail, ensure, Context, Result};

mod server;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let addr = init_handshake();

    let addr = match addr {
        Ok(addr) => addr,
        Err(err) => {
            println!("{:?}", err);
            bail!("init error");
        }
    };

    let cert = std::env::var("PLUGIN_CLIENT_CERT").wrap_err("PLUGIN_CLIENT_CERT not found")?;

    tonic::transport::Server::builder()
        .add_service(server::tfplugin6::provider_server::ProviderServer::new(
            server::MyProvider,
        ))
        .serve(addr)
        .await
        .wrap_err("failed to start server")?;

    Ok(())
}

fn init_handshake() -> Result<SocketAddr> {
    // https://github.com/hashicorp/go-plugin/blob/8d2aaa458971cba97c3bfec1b0380322e024b514/docs/internals.md

    let min_port = env::var("PLUGIN_MIN_PORT")
        .wrap_err("PLUGIN_MIN_PORT not found")?
        .parse::<u16>()
        .wrap_err("PLUGIN_MIN_PORT not an int")?;
    let max_port = env::var("PLUGIN_MAX_PORT")
        .wrap_err("PLUGIN_MAX_PORT not found")?
        .parse::<u16>()
        .wrap_err("PLUGIN_MAX_PORT not an int")?;

    let port = min_port + 15; // chosen by a d20, lol
    ensure!(port < max_port);

    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let addr = SocketAddr::new(addr, port);

    const VERSION: u8 = 6;

    println!("1|{VERSION}|tcp|{addr}|grpc");

    Ok(addr)
}
