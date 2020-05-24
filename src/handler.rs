use crate::grpc::{
    AddRequest, AddResponse, DeleteRequest, DeleteResponse, DescribeRequest, DescribeResponse,
    ListRequest, SearchRequest, SearchResponse,
};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

use crate::grpc::embedding_db_server::EmbeddingDb;

#[derive(Debug, Default)]
pub struct EmbeddingDBHandler {}

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
