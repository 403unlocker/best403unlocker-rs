use reqwest::dns::{Resolve, Resolving};
use std::error::Error;
use std::future::Future;
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::vec;
use tokio::sync::Mutex;
use trust_dns_resolver::config::{NameServerConfigGroup, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

pub struct CustomDnsResolver {
    resolver: Arc<Mutex<TokioAsyncResolver>>,
}

impl CustomDnsResolver {
    pub fn new(dns_resolver: IpAddr) -> Self {
        let name_servers = NameServerConfigGroup::from_ips_clear(&[dns_resolver], 53, false);
        let resolver_config = ResolverConfig::from_parts(None, vec![], name_servers);
        let resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());

        CustomDnsResolver {
            resolver: Arc::new(Mutex::new(resolver)),
        }
    }
}

impl Resolve for CustomDnsResolver {
    fn resolve(&self, name: reqwest::dns::Name) -> Resolving {
        let resolver = self.resolver.clone();

        let fut = async move {
            let name_str = name.as_str();
            let resolver = resolver.lock().await;
            let response = resolver.lookup_ip(name_str).await?;
            let addrs: Vec<SocketAddr> = response.iter().map(|ip| SocketAddr::new(ip, 0)).collect();
            Ok(Box::new(addrs.into_iter()) as Box<dyn Iterator<Item = SocketAddr> + Send>)
        };

        Box::pin(fut)
            as Pin<
                Box<
                    dyn Future<
                            Output = Result<
                                Box<dyn Iterator<Item = SocketAddr> + Send>,
                                Box<dyn Error + Send + Sync>,
                            >,
                        > + Send,
                >,
            >
    }
}
