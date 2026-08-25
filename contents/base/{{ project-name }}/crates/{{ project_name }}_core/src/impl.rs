use tonic::{Request, Response, Status};

use crate::proto::{{ project_name }}_server::{{ ProjectName }};
use crate::proto::{
    Create{{ EntityName }}Request, Delete{{ EntityName }}Request, Delete{{ EntityName }}Response, {{ EntityName }}, Get{{ EntityName }}Request, List{{ EntityName }}sRequest,
    List{{ EntityName }}sResponse, Update{{ EntityName }}Request,
};
use crate::{{ ProjectName }}Core;

fn internal(err: anyhow::Error) -> Status {
    tracing::error!("store error: {err:#}");
    Status::internal("internal error")
}

#[tonic::async_trait]
impl {{ ProjectName }} for {{ ProjectName }}Core {
    async fn create_{{ entity_name }}(&self, request: Request<Create{{ EntityName }}Request>) -> Result<Response<{{ EntityName }}>, Status> {
        let req = request.into_inner();
        let created = self.store.create(&req.display_name).await.map_err(internal)?;
        Ok(Response::new(created))
    }

    async fn get_{{ entity_name }}(&self, request: Request<Get{{ EntityName }}Request>) -> Result<Response<{{ EntityName }}>, Status> {
        let req = request.into_inner();
        match self.store.get(&req.id).await.map_err(internal)? {
            Some(entity) => Ok(Response::new(entity)),
            None => Err(Status::not_found(format!("no {{ entity_name }} with id {}", req.id))),
        }
    }

    async fn list_{{ entity_name }}s(
        &self,
        _request: Request<List{{ EntityName }}sRequest>,
    ) -> Result<Response<List{{ EntityName }}sResponse>, Status> {
        let items = self.store.list().await.map_err(internal)?;
        Ok(Response::new(List{{ EntityName }}sResponse { items }))
    }

    async fn update_{{ entity_name }}(&self, request: Request<Update{{ EntityName }}Request>) -> Result<Response<{{ EntityName }}>, Status> {
        let req = request.into_inner();
        match self.store.update(&req.id, &req.display_name).await.map_err(internal)? {
            Some(entity) => Ok(Response::new(entity)),
            None => Err(Status::not_found(format!("no {{ entity_name }} with id {}", req.id))),
        }
    }

    async fn delete_{{ entity_name }}(
        &self,
        request: Request<Delete{{ EntityName }}Request>,
    ) -> Result<Response<Delete{{ EntityName }}Response>, Status> {
        let req = request.into_inner();
        if self.store.delete(&req.id).await.map_err(internal)? {
            Ok(Response::new(Delete{{ EntityName }}Response {}))
        } else {
            Err(Status::not_found(format!("no {{ entity_name }} with id {}", req.id)))
        }
    }
}
