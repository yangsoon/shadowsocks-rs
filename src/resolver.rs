use std::io;
use std::net::IpAddr;

use crate::util::other;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use trust_dns_resolver::{AsyncResolver, TokioAsyncResolver};

// pub async fn resolve(host: &str) -> io::Result<IpAddr> {
//     let host = format!("{}:0", host);
//     let ip = tokio::task::spawn_blocking(move || match host[..].to_socket_addrs() {
//         Ok(it) => {
//             let mut it = it.filter(|x| match x.ip() {
//                 IpAddr::V4(_) => true,
//                 IpAddr::V6(_) => false,
//             });
//             if let Some(addr) = it.next() {
//                 Ok(addr.ip())
//             } else {
//                 Err(other("no ip return"))
//             }
//         }
//         Err(e) => Err(e),
//     })
//     .await?;
//     ip
// }

lazy_static! {
    // setup the global Resolver
    static ref GLOBAL_DNS_RESOLVER: TokioAsyncResolver = {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let resolver = AsyncResolver::from_system_conf(runtime.handle().clone());
        runtime
            .block_on(resolver)
            .expect("failed to create resolver")
    };
}

pub async fn resolve(host: &str) -> io::Result<IpAddr> {
    match GLOBAL_DNS_RESOLVER.lookup_ip(host).await {
        Ok(r) => {
            if let Some(addr) = r.iter().next() {
                Ok(addr)
            } else {
                Err(other("no ip return"))
            }
        }
        Err(e) => Err(other(&format!("resolve fail: {:?}", e))),
    }
}
