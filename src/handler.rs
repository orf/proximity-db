use crate::grpc::{
    AddRequest, AddResponse, DeleteRequest, DeleteResponse, DescribeRequest, DescribeResponse,
    ListRequest, SearchRequest, SearchResponse,
};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

use crate::grpc::embedding_db_server::EmbeddingDb;

pub struct EmbeddingDBHandler {}

impl EmbeddingDBHandler {
    pub fn new() -> Self {
        EmbeddingDBHandler {}
    }
}


#[tonic::async_trait]
impl EmbeddingDb for EmbeddingDBHandler {
    type SearchStream = mpsc::Receiver<Result<SearchResponse, Status>>;

    async fn search(
        &self,
        _request: Request<SearchRequest>,
    ) -> Result<Response<Self::SearchStream>, Status> {
        unimplemented!()
    }

    async fn add(&self, _request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        unimplemented!()
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
