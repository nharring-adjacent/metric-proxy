use metricproxy::metric_proxy_server::{MetricProxy, MetricProxyServer};
use metricproxy::{DataType,DescribeMetricsRequest, MetricsDescription, MetricType, PageInfo, SortOrder, TagValues};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use std::collections::HashMap;
use std::option::{Option};

mod metricproxy {
    include!("metricproxy/metricproxy.rs");
    pub(crate) const FILE_DESCRIPTOR_SET: &'static [u8] =
        include_bytes!("metricproxy/metricproxy_descriptor.bin");
}

#[derive(Debug)]
pub struct MetricProxyService {
    metrics: HashMap<String, MetricType>,
}

#[tonic::async_trait]
impl MetricProxy for MetricProxyService {
    async fn describe_metrics(&self, request: Request<DescribeMetricsRequest>) -> Result<Response<MetricsDescription>, Status> {
        let req = request.into_inner();
        println!("Got request: {:?}", req);

        if req.filter == *"" {
            println!("Empty filter, returning error response");
            let resp = MetricsDescription {
                error: true,
                error_info: "empty string is an invalid filter".to_string(),
                page_info: Option::None,
                metrics: HashMap::new(),
            };
            return Ok(Response::new(resp));
        }
        if req.filter == *"*" {
            println!("Wildcard filter, returning all metrics");
            let pi = PageInfo {
                count: self.metrics.keys().len() as i32,
                offset: 0,
                order: SortOrder::Undef as i32,
            };
            let resp = MetricsDescription {
                error: false,
                error_info: "".to_string(),
                page_info: Option::Some(pi),
                metrics: self.metrics.clone(),
            };
            return Ok(Response::new(resp));
        }
        println!("Filter: {}, returning prefix matches", req.filter);
        let mut metrics: HashMap<String, MetricType> = HashMap::new();
        for (key, val) in self.metrics.iter() {
            if key.starts_with(&req.filter) {
                metrics.insert(key.to_string(), val.clone());
            }
        }
        let mut resp = MetricsDescription {
            error: false,
            error_info: "".to_string(),
            page_info: Option::Some(PageInfo::default()),
            metrics,
        };

        let pi = PageInfo {
            count: resp.metrics.keys().len() as i32,
            offset: 0,
            order: SortOrder::Undef as i32,
        };
        resp.page_info = Option::Some(pi);
        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    let metrics = populate_map();
    let metric_proxy = MetricProxyService {
        metrics,
    };

    println!("MetricProxyServer listening on: {}", addr);

    let svc = MetricProxyServer::new(metric_proxy);
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(metricproxy::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    Server::builder().add_service(svc).add_service(reflection).serve(addr).await?;

    Ok(())
}

fn populate_map() -> HashMap<String, MetricType> {
    let mut metrics = HashMap::new();
    let mut tvs: HashMap<String, TagValues> = HashMap::new();
    tvs.insert("tag_1".to_string(), TagValues::default());
    let mt = MetricType {
        id: "a".to_string(),
        name: "a metric".to_string(),
        r#type: DataType::Count as i32,
        tags: vec!["tag_1".to_string()],
        tag_values: tvs,
    };
    metrics.insert("a".to_string(), mt);
    metrics
}
