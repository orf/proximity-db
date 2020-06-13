use proximity_grpc::{
    AddRequest, AddResponse, DeleteRequest, DeleteResponse, DescribeRequest, DescribeResponse,
    ListRequest, Point as GrpcPoint, SearchRequest, SearchResponse,
};
use tokio::sync::mpsc;
use tonic::{Code, Request, Response, Status};

use crate::sky::{Metrics, Sky};
use proximity_grpc::proximity_db_server::ProximityDb;
use std::sync::Arc;

#[derive(Default)]
pub struct ProximityDBHandler {
    sky: Arc<Sky>,
}

impl ProximityDBHandler {
    pub fn new(sky: Sky) -> Self {
        ProximityDBHandler { sky: sky.into() }
    }
}

#[tonic::async_trait]
impl ProximityDb for ProximityDBHandler {
    type SearchStream = mpsc::UnboundedReceiver<Result<SearchResponse, Status>>;

    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<Self::SearchStream>, Status> {
        let search_request = request.into_inner();

        if search_request.point.is_none() {
            return Err(Status::new(Code::InvalidArgument, "No point given"));
        }

        let sky_reference = self.sky.clone();

        let (tx, rx) = mpsc::unbounded_channel();

        tokio::task::spawn_blocking(move || {
            match sky_reference.query(
                search_request.name,
                search_request.distance,
                search_request.point.unwrap().coords,
            ) {
                Err(e) => {
                    tx.send(Err(e.into())).unwrap();
                }
                Ok(query_iterator) => {
                    for (distance, coords) in query_iterator {
                        if let Err(_) = tx.send(Ok(SearchResponse {
                            distance,
                            point: Some(GrpcPoint { coords }),
                        })) {
                            break;
                        }
                    }
                }
            };
        });
        Ok(Response::new(rx))
    }

    async fn add(
        &self,
        request: Request<tonic::Streaming<AddRequest>>,
    ) -> Result<Response<AddResponse>, Status> {
        let mut stream = request.into_inner();
        let sky = self.sky.clone();
        let mut total_added = 0;
        while let Some(add_request) = stream.message().await? {
            total_added += sky.add(
                add_request.name,
                add_request.points.into_iter().map(|p| p.coords).collect(),
            )?;
        }
        Ok(Response::new(AddResponse {
            total_added: total_added as u64,
        }))
    }

    async fn delete(
        &self,
        _request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        unimplemented!()
    }

    type ListStream = mpsc::UnboundedReceiver<Result<DescribeResponse, Status>>;

    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let prefix = request.into_inner().prefix;
        let (tx, rx) = mpsc::unbounded_channel();

        for metric in self.sky.list(&prefix) {
            if let Err(_) = tx.send(Ok(metric.into())) {
                break;
            }
        }

        Ok(Response::new(rx))
    }

    async fn describe(
        &self,
        request: Request<DescribeRequest>,
    ) -> Result<Response<DescribeResponse>, Status> {
        let name = request.into_inner().name;
        let metrics = self.sky.describe(&name)?;

        Ok(Response::new(metrics.into()))
    }
}

impl Into<DescribeResponse> for Metrics {
    fn into(self) -> DescribeResponse {
        DescribeResponse {
            name: self.name,
            count: self.count as u64,
            dimensions: self.dimensions as u64,
            memory_size: self.memory_size as u64,
        }
    }
}

// https://github.com/hyperium/tonic/blob/6f378e2bd0cdf3a1a3df87e1feff842a8a599142/tonic-health/src/server.rs#L156
