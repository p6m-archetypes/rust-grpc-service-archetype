pub mod proto {
    tonic::include_proto!("{{ prefix_name }}_{{ suffix_name }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("{{ prefix_name }}_{{ suffix_name }}_descriptor");
}
