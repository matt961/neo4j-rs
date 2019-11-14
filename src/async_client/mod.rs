use std::fmt::Display;
use tokio::prelude::*;

struct AsyncClient {
    stream: tokio::net::TcpStream,
}

impl AsyncClient {
    async fn connect(addr: &str) -> Result<Self, AsyncClientError> {
        let mut stream = tokio::net::TcpStream::connect(addr).await?;

        // perform Handshake (see https://boltprotocol.org/v1/#handshake)
        stream.write_all(&MAGIC).await?;
        stream.write_all(&SUPPORTED_VERSIONS).await?;

        let mut version_buf = [0_u8; 4];
        stream.read_exact(&mut version_buf).await?;
        let version = u32::from_be_bytes(version_buf);

        if version == 0 {
            return Err(AsyncClientError::new(AsyncClientErrorKind::Version));
        }

        // TODO finish impl

        Ok(AsyncClient { stream })
    }
}

#[derive(Debug, PartialEq)]
struct AsyncClientError {
    kind: AsyncClientErrorKind,
}

#[derive(Debug, PartialEq)]
enum AsyncClientErrorKind {
    IO,
    Version,
}

impl AsyncClientError {
    fn new(kind: AsyncClientErrorKind) -> Self {
        AsyncClientError { kind }
    }
}

impl Display for AsyncClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for AsyncClientErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<tokio::io::Error> for AsyncClientError {
    fn from(_: tokio::io::Error) -> Self {
        AsyncClientError::new(AsyncClientErrorKind::IO)
    }
}

impl std::error::Error for AsyncClientError {}

const MAGIC: [u8; 4] = [0x60, 0x60, 0xb0, 0x17];

// 4 versions specified as u32
const SUPPORTED_VERSIONS: [u8; 4 * 4] = [0, 0, 0, 0x1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
