use tonic::{Request, Response, Status};

use crate::proto::{{ prefix_name }}_{{ suffix_name }}_server::{{ PrefixName }}{{ SuffixName }};
use crate::proto::{
    Create{{ PrefixName }}Request, Delete{{ PrefixName }}Request, Delete{{ PrefixName }}Response, {{ PrefixName }}, Get{{ PrefixName }}Request, List{{ PrefixName }}sRequest,
    List{{ PrefixName }}sResponse, Update{{ PrefixName }}Request,
};
use crate::{{ PrefixName }}{{ SuffixName }}Core;

fn internal(err: anyhow::Error) -> Status {
    tracing::error!("store error: {err:#}");
    Status::internal("internal error")
}

#[tonic::async_trait]
impl {{ PrefixName }}{{ SuffixName }} for {{ PrefixName }}{{ SuffixName }}Core {
    async fn create_{{ prefix_name }}(&self, request: Request<Create{{ PrefixName }}Request>) -> Result<Response<{{ PrefixName }}>, Status> {
        let req = request.into_inner();
        let created = self.store.create(&req.display_name).await.map_err(internal)?;
        Ok(Response::new(created))
    }

    async fn get_{{ prefix_name }}(&self, request: Request<Get{{ PrefixName }}Request>) -> Result<Response<{{ PrefixName }}>, Status> {
        let req = request.into_inner();
        match self.store.get(&req.id).await.map_err(internal)? {
            Some(entity) => Ok(Response::new(entity)),
            None => Err(Status::not_found(format!("no {{ prefix_name }} with id {}", req.id))),
        }
    }

    async fn list_{{ prefix_name }}s(
        &self,
        _request: Request<List{{ PrefixName }}sRequest>,
    ) -> Result<Response<List{{ PrefixName }}sResponse>, Status> {
        let items = self.store.list().await.map_err(internal)?;
        Ok(Response::new(List{{ PrefixName }}sResponse { items }))
    }

    async fn update_{{ prefix_name }}(&self, request: Request<Update{{ PrefixName }}Request>) -> Result<Response<{{ PrefixName }}>, Status> {
        let req = request.into_inner();
        match self.store.update(&req.id, &req.display_name).await.map_err(internal)? {
            Some(entity) => Ok(Response::new(entity)),
            None => Err(Status::not_found(format!("no {{ prefix_name }} with id {}", req.id))),
        }
    }

    async fn delete_{{ prefix_name }}(
        &self,
        request: Request<Delete{{ PrefixName }}Request>,
    ) -> Result<Response<Delete{{ PrefixName }}Response>, Status> {
        let req = request.into_inner();
        if self.store.delete(&req.id).await.map_err(internal)? {
            Ok(Response::new(Delete{{ PrefixName }}Response {}))
        } else {
            Err(Status::not_found(format!("no {{ prefix_name }} with id {}", req.id)))
        }
    }
}
