// pub const SPIDERS_WEB_PORT_BASE: u16 = 63000;
pub const SPIDERS_GRPC_PORT_BASE: u16 = 29000;
// pub const SPIDERS_WEBSOCKET_PORT_BASE: u16 = 55000;

pub mod server {
    tonic::include_proto!("server");
}
