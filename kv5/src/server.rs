use anyhow::Result;
use kv5::{MemTable, ProstServerStream, Service, ServiceInner, YamuxCtrl};
use tokio::net::TcpListener;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    let service: Service = ServiceInner::new(MemTable::new()).into();
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            YamuxCtrl::new_server(stream, None, move |stream| {
                let svc1 = svc.clone();
                async move {
                    let stream = ProstServerStream::new(stream.compat(), svc1.clone());
                    stream.process().await.unwrap();
                    Ok(())
                }
            });
        });

    }
}
