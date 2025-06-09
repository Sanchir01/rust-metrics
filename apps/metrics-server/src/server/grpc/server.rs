use tonic::{Request, Response, Status};
use candles_proto::metrics::{AddMetricRequest, AddMetricResponse,metrics_server::Metrics};


#[derive(Debug,Default)]
pub struct MetricsGrpcServer {}

#[tonic::async_trait]
impl Metrics for MetricsGrpcServer {
    async fn add_metric(
        &self,
        request: Request<AddMetricRequest>,
    ) -> Result<Response<AddMetricResponse>, Status> {
        let req = request.into_inner();
        println!("📥 AddMetricRequest received: {:?}", req);


        let response = AddMetricResponse {
            url: req.url,
            timestamp: chrono::Utc::now().timestamp(),
            http_metrics: None,
            performance_metrics: None,
            seo_metrics: None,
            error: String::new(),
        };

        Ok(Response::new(response))
    }
}
impl MetricsGrpcServer {
        pub fn new() -> Self {
            Self { }
        }
}