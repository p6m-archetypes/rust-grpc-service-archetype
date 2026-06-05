use tonic::{Request, Response, Status};
use tracing::info;

use crate::{{ PrefixName }}{{ SuffixName }}Core;
use crate::proto::{{ prefix_name }}_{{ suffix_name }}_server::{{ PrefixName }}{{ SuffixName }};
use crate::proto::{
    {{ PrefixName }}, Get{{ PrefixName }}Request, List{{ PrefixName }}sRequest, List{{ PrefixName }}sResponse,
};

#[tonic::async_trait]
impl {{ PrefixName }}{{ SuffixName }} for {{ PrefixName }}{{ SuffixName }}Core {
    async fn create_{{ prefix_name }}(
        &self,
        request: Request<{{ PrefixName }}>,
    ) -> Result<Response<{{ PrefixName }}>, Status> {
        let _req = request.into_inner();
        info!("create_{{ prefix_name }} called");
        Err(Status::unimplemented("create_{{ prefix_name }} not implemented"))
    }

    async fn get_{{ prefix_name }}(
        &self,
        request: Request<Get{{ PrefixName }}Request>,
    ) -> Result<Response<{{ PrefixName }}>, Status> {
        let _req = request.into_inner();
        info!("get_{{ prefix_name }} called");
        Err(Status::unimplemented("get_{{ prefix_name }} not implemented"))
    }

    async fn list_{{ prefix_name }}s(
        &self,
        request: Request<List{{ PrefixName }}sRequest>,
    ) -> Result<Response<List{{ PrefixName }}sResponse>, Status> {
        let _req = request.into_inner();
        info!("list_{{ prefix_name }}s called");
        Err(Status::unimplemented("list_{{ prefix_name }}s not implemented"))
    }

    async fn update_{{ prefix_name }}(
        &self,
        request: Request<{{ PrefixName }}>,
    ) -> Result<Response<{{ PrefixName }}>, Status> {
        let _req = request.into_inner();
        info!("update_{{ prefix_name }} called");
        Err(Status::unimplemented("update_{{ prefix_name }} not implemented"))
    }
}
