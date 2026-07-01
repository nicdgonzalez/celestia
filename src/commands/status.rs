use std::io::{self, Read, Write};
use std::net::{IpAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use anyhow::{Context as _, anyhow, bail};
use colored::Colorize as _;
use tracing::debug;
use trust_dns_resolver::TokioAsyncResolver;

use crate::commands::prelude::*;
use crate::package::server::{GetValueError, Properties};
use crate::varint::{self, ReadExt};

/// Arguments for the `status` subcommand.
#[derive(clap::Args)]
pub struct Status {
    /// Maximum number of seconds to wait before failing to connect to server
    #[clap(long, default_value = "30")]
    timeout: u64,

    /// Domain name or IP address of target server
    #[clap(long, short = 'H')]
    host: Option<String>,

    /// Port number where target server is listening from
    #[clap(long, short)]
    port: Option<u16>,
}

impl Run for Status {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let timeout = Duration::from_secs(self.timeout);
        let properties = ctx.package().server().properties();

        let (ip, port) = match self.host.as_deref() {
            Some(host) => resolve_host(host, self.port, &properties).await?,
            None => resolve_local_server(self.port, &properties)?,
        };

        let server_address = format!("{ip}:{port}");
        let mut socket = server_address
            .to_socket_addrs()
            .context("failed to resolve server address")?
            .find_map(|addr| TcpStream::connect_timeout(&addr, timeout).ok())
            .context("failed to connect to Minecraft server")?;

        let response =
            server_list_ping(&mut socket).context("failed to get response from server")?;

        print_response(&response, &server_address);

        Ok(())
    }
}

fn print_response(response: &StatusResponse, server_address: &str) {
    let motd = response.description.as_ref().map_or_else(
        || "None".to_owned(),
        |d| match d {
            Description::Simple(text) | Description::Colored { text, .. } => text.clone(),
        },
    );

    let player_count = response
        .players
        .as_ref()
        .map_or_else(|| "???".to_owned(), |p| p.online.to_string());

    let mut stdout = io::stdout().lock();
    writeln!(stdout, "{}: {}", "Server Address".bold(), server_address).ok();

    if !motd.is_empty() {
        writeln!(stdout, "{}: {}", "MOTD".bold(), motd).ok();
    }

    writeln!(stdout, "{}: {}", "Players Online".bold(), player_count).ok();

    if let Some(sample) = response.players.as_ref().and_then(|p| p.sample.as_ref()) {
        for player in sample {
            writeln!(stdout, "{} ({})", player.name, player.id).ok();
        }
    }
}

#[expect(dead_code)]
#[derive(serde::Deserialize)]
struct StatusResponse {
    description: Option<Description>,
    favicon: Option<String>,
    players: Option<Players>,
    version: Version,
}

#[expect(dead_code)]
#[derive(Clone, serde::Deserialize)]
#[serde(untagged)]
enum Description {
    Simple(String),
    Colored { color: Option<String>, text: String },
}

#[expect(dead_code)]
#[derive(serde::Deserialize)]
struct Players {
    max: u32,
    online: u32,
    sample: Option<Vec<Sample>>,
}

#[derive(serde::Deserialize)]
struct Sample {
    name: String,
    id: String,
}

#[expect(dead_code)]
#[derive(serde::Deserialize)]
struct Version {
    name: String,
    protocol: i32,
}

async fn resolve_host(
    host: &str,
    port: Option<u16>,
    properties: &Properties,
) -> anyhow::Result<(String, u16)> {
    if let Ok(addr) = host.parse::<IpAddr>() {
        Ok((
            addr.to_canonical().to_string(),
            resolve_port(port, properties)?,
        ))
    } else {
        resolve_srv(host).await
    }
}

async fn resolve_srv(host: &str) -> anyhow::Result<(String, u16)> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    let srv_name = format!("_minecraft._tcp.{host}");

    match resolver.srv_lookup(&srv_name).await {
        Ok(records) => {
            let record = records.iter().next().context("empty SRV response")?;

            Ok((
                record.target().to_utf8().trim_end_matches('.').to_string(),
                record.port(),
            ))
        }
        Err(_) => {
            // Fall back to default Minecraft port.
            Ok((host.to_string(), 25565))
        }
    }
}

fn resolve_local_server(
    port: Option<u16>,
    properties: &Properties,
) -> anyhow::Result<(String, u16)> {
    let addr = properties
        .get::<String>("server-ip")
        .context("failed to read server.properties")?
        .map_or_else(
            || "127.0.0.1".to_owned(),
            |value| {
                if value.is_empty() {
                    "127.0.0.1".to_owned()
                } else {
                    value
                }
            },
        )
        .parse::<IpAddr>()
        .context("invalid value for 'server-ip' in server.properties")?;

    Ok((
        addr.to_canonical().to_string(),
        resolve_port(port, properties)?,
    ))
}

fn resolve_port(port: Option<u16>, properties: &Properties) -> anyhow::Result<u16> {
    if let Some(port) = port {
        return Ok(port);
    }

    match properties.get::<u16>("server-port") {
        Ok(Some(port)) => Ok(port),
        Ok(None) => Ok(25565),
        Err(err) => match err {
            GetValueError::Io { .. } => Err(err).context("failed to read server.properties")?,
            GetValueError::Parse => bail!("invalid value for 'server-port' in server.properties"),
        },
    }
}

fn server_list_ping(socket: &mut TcpStream) -> anyhow::Result<StatusResponse> {
    let peer = socket
        .peer_addr()
        .context("failed to get server endpoint")?;

    let ip = peer.ip().to_canonical().to_string();
    let port = peer.port();

    let packet = handshake_packet(&ip, port)?;
    socket
        .write_all(&packet)
        .context("failed to send handshake packet")?;

    let packet = status_request_packet();
    socket
        .write_all(&packet)
        .context("failed to send status request packet")?;

    get_status_response(socket)
}

fn handshake_packet(host: &str, port: u16) -> anyhow::Result<Vec<u8>> {
    let packet_id = varint::encode(0x00);
    let protocol_version = varint::encode(0); // This value is not important for the ping.
    let server_address_length = i32::try_from(host.len())
        .map(varint::encode)
        // The maximum length of a valid hostname is 253.
        // https://en.m.wikipedia.org/wiki/Hostname#Syntax
        .context("failed to fit hostname length in an i32")?;
    let server_port_length = std::mem::size_of_val(&port);
    let next_state = varint::encode(1);

    let packet_length = packet_id.len()
        + protocol_version.len()
        + server_address_length.len()
        + host.len()
        + server_port_length
        + next_state.len();

    let packet_length_encoded = i32::try_from(packet_length)
        .map(varint::encode)
        .context("failed to fit packet length in an i32")?;

    let capacity = packet_length_encoded.len() + packet_length;

    let mut packet = Vec::with_capacity(capacity);
    packet.extend(packet_length_encoded);
    packet.extend(packet_id);
    packet.extend(protocol_version);
    packet.extend(server_address_length);
    packet.extend(host.as_bytes());
    packet.extend(port.to_be_bytes());
    packet.extend(next_state);
    debug!("Handshake packet: {packet:?}");

    Ok(packet)
}

fn status_request_packet() -> Vec<u8> {
    let packet_id = varint::encode(0x00);
    let packet_length = packet_id.len(); // This request has no additional data.
    let packet_length_encoded = i32::try_from(packet_length).map(varint::encode).unwrap();
    let capacity = packet_length_encoded.len() + packet_length;

    let mut packet = Vec::with_capacity(capacity);
    packet.extend(packet_length_encoded);
    packet.extend(packet_id);
    tracing::debug!("Status Request packet: {packet:?}");

    packet
}

fn get_status_response(socket: &mut TcpStream) -> anyhow::Result<StatusResponse> {
    if let Err(err) = socket.read_varint_i32() {
        if let varint::ReadVarIntError::ReadFailed { source } = &err {
            // Indicates there *is* a server listening to requests at this address,
            // but it probably disregarded our request because it's not a Minecraft server.
            if source
                .downcast_ref::<std::io::Error>()
                .filter(|e| e.kind() == std::io::ErrorKind::UnexpectedEof)
                .is_some()
            {
                return Err(anyhow::anyhow!(
                    "no response from server. are you sure this is a Minecraft server?"
                ));
            }
        }

        return Err(err.into());
    }

    let packet_id = socket
        .read_varint_i32()
        .context("failed to get packet ID")?;

    if packet_id != 0x00 {
        return Err(anyhow!("expected the packet ID to be 0, got {packet_id}"));
    }

    let data_length = socket
        .read_varint_i32()
        .context("failed to get data length")?;

    let size = usize::try_from(data_length).expect("i32 overflowed usize");
    let mut buffer = vec![0u8; size];
    socket
        .read_exact(&mut buffer)
        .context("failed to get data")?;

    let content = String::from_utf8(buffer).context("expected response to be valid UTF-8")?;

    let data: StatusResponse =
        serde_json::from_str(&content).context("failed to parse response body")?;

    Ok(data)
}
