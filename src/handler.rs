use crate::grpc::{
    AddRequest, AddResponse, DeleteRequest, DeleteResponse, DescribeRequest, DescribeResponse,
    ListRequest, SearchRequest, SearchResponse,
};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

use crate::grpc::embedding_db_server::EmbeddingDb;
use crate::sky::Sky;
use crossbeam_channel::unbounded;
use std::sync::Arc;

#[derive(Default)]
pub struct EmbeddingDBHandler {
    sky: Arc<Sky>,
}

impl EmbeddingDBHandler {
    pub fn new(sky: Sky) -> Self {
        EmbeddingDBHandler { sky: sky.into() }
    }
}

#[tonic::async_trait]
impl EmbeddingDb for EmbeddingDBHandler {
    type SearchStream = mpsc::UnboundedReceiver<Result<SearchResponse, Status>>;

    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<Self::SearchStream>, Status> {
        let search_request = request.into_inner();
        let sky_reference = self.sky.clone();

        let (tx, rx) = mpsc::unbounded_channel();

        tokio::task::spawn_blocking(move || {
            let (sync_sender, sync_receiver) = unbounded();
            if let Err(e) = sky_reference.query(
                search_request.name,
                search_request.distance,
                search_request.point,
                sync_sender,
            ) {
                tx.send(Err(e.into())).ok();
            }
            for (distance, point) in sync_receiver.iter() {
                if let Err(_) = tx.send(Ok(SearchResponse { distance, point })) {
                    break;
                }
            }
        });

        Ok(Response::new(rx))
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let add_request = request.into_inner();
        let sky = self.sky.clone();
        sky.add(add_request.name, add_request.point)?;
        Ok(Response::new(AddResponse {}))
    }

    async fn delete(
        &self,
        _request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        unimplemented!()
    }

    type ListGroupsStream = mpsc::Receiver<Result<DescribeResponse, Status>>;

    async fn list_groups(
        &self,
        _request: Request<ListRequest>,
    ) -> Result<Response<Self::ListGroupsStream>, Status> {
        unimplemented!()
    }

    async fn describe_group(
        &self,
        _request: Request<DescribeRequest>,
    ) -> Result<Response<DescribeResponse>, Status> {
        unimplemented!()
    }
}

// https://github.com/hyperium/tonic/blob/6f378e2bd0cdf3a1a3df87e1feff842a8a599142/tonic-health/src/server.rs#L156
