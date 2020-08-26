use daydream_druid::rmain;
use druid::PlatformError;

/// Wrapper for cargo run as the web stuff needs a lib to run
fn main() -> Result<(), PlatformError> {
    rmain()
}
