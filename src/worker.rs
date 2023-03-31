use etcd_client::{Client, LockOptions};
use log::info;
use std::time::Duration;

use crate::errors::AnyError;

pub struct LeaseWorker;

impl LeaseWorker {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self, addr: &str) -> Result<i32, AnyError> {
        let mut client = Client::connect([addr], None).await?;
        let response = client.lease_grant(15, None).await?;

        info!(
            "grant a lease with id {:?}, ttl {:?}",
            response.id(),
            response.ttl()
        );

        let lease_id = response.id();

        for i in 0..1023 {
            let lock = format!("id-gen-worker-{i}");
            info!("try to lock with name \'{lock}\' and lease {lease_id}");

            let options = LockOptions::new().with_lease(lease_id);

            let Ok(_) = client.lock(lock, Some(options)).await else {
                continue;
            };

            tokio::spawn(async move {
                let (mut keeper, mut stream) = client.lease_keep_alive(lease_id).await.unwrap();

                loop {
                    info!("lease {:?} keep alive start", lease_id);

                    keeper.keep_alive().await.unwrap();

                    let Some(response) = stream.message().await.unwrap() else {
                        continue;
                    };

                    info!(
                        "lease {:?} keep alive, new ttl {:?}s",
                        response.id(),
                        response.ttl()
                    );

                    tokio::time::sleep(Duration::from_secs((response.ttl() / 3) as u64)).await;
                }
            });

            return Ok(i);
        }

        Err(etcd_client::Error::ElectError("unable to find id".to_string()).into())
    }
}
