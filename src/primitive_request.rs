use std::path::PathBuf;

pub struct PrimitiveRequest {
    pub request_id: String,
    pub input_file_path: PathBuf,
    pub num_shapes: u64,
    pub max_age: u64,
    pub scale_to: u64,
    pub seed: u64,
    pub shape: String
}