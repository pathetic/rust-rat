pub mod buffers;
pub mod commands;
pub mod packet;

pub const RSA_BITS: usize = 1024;
pub const ENC_TOK_LEN: usize = 32;
pub const SECRET_LEN: usize = 32;
pub const NONCE_LEN: usize = 24;

pub struct ClientConfig {
    pub ip: String,
    pub port: u16,
    pub mutex_check: bool,
    pub unattended_mode: bool,
    pub startup: bool
}