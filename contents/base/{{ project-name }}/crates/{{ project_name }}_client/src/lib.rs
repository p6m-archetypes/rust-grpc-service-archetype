// Generated tonic stubs: `connect()` returns Result<_, tonic::transport::Error>, whose Err
// variant trips clippy::result_large_err under -D warnings. Not ours to fix — allow it here.
#[allow(clippy::result_large_err)]
pub mod proto {
    tonic::include_proto!("{{ project_name }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ project_name }}_descriptor");
}
