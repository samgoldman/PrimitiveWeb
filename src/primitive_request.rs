use std::path::PathBuf;

pub struct PrimitiveRequest {
    pub request_id: String,
    pub input_file_path: PathBuf,
    pub num_shapes: u32,
    pub max_age: u32,
    pub scale_to: u32,
    pub seed: u64,
    pub shape: String
}