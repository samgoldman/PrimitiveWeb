use rocket_contrib::json::JsonValue;
use rocket::http::ContentType;
use nanoid::nanoid;
use crossbeam_queue::SegQueue;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, MultipartFormDataError};
use rocket::Data;
use std::{fs, env};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use rocket::State;
use crate::primitive_request::PrimitiveRequest;
use crate::Q;

const VALID_SHAPES: [&str; 6] = ["TRIANGLE", "CUBIC", "QUADRATIC", "RECTANGLE", "ELLIPSE", "MIXED"];
const MAX_IMAGE_SIZE: u64 = 32; // MB

const NUM_SHAPES_DEFAULT: u64 = 500;
const MAX_AGE_DEFAULT: u64 = 100;
const SCALE_TO_DEFAULT: u64 = 100;
const SEED_DEFAULT: u64 = 0;
const SHAPE_DEFAULT: &str = VALID_SHAPES[0];

const NUM_SHAPES_MAX: u64 = 2000;
const MAX_AGE_MAX: u64 = 200;

fn extract_uint64(multipart_form_data: &mut MultipartFormData, name: &str, default: u64) -> Option<u64> {
    match multipart_form_data.texts.remove(name) {
        Some(mut values) => {
            let text_value = values.remove(0).text;
            let parse_result = u64::from_str(text_value.as_ref());

            match parse_result {
                Ok(value) => Some(value),
                Err(_err) => None
            }
        },
        None => Some(default)
    }
}

///
/// Handler for submitting images for processing.
///
/// POST Parameters:
///     image: the image to process (required, max MAX_IMAGE_SIZE MB)
///     num_shapes: the number of shapes to use for the primitive image (default 500, max 2000)
///     max_age: Maximum age for each hill climbing attempt (default 100, max 200)
///     scale_to: The value to scale the image's largest dimension to (default 100)
///     seed: the random seed. 0 picks a seed based on the time (default 0)
///     shape: the shape to use. (default TRIANGLE)
///
#[post("/submit", data = "<data>")]
pub fn submit(content_type: &ContentType, data: Data, queue: State<&Q>) -> JsonValue {
    let request_id = nanoid!(42);

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("image")
            .size_limit(MAX_IMAGE_SIZE * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        MultipartFormDataField::text("num_shapes"),
        MultipartFormDataField::text("max_age"),
        MultipartFormDataField::text("scale_to"),
        MultipartFormDataField::text("seed"),
        MultipartFormDataField::text("shape")
    ]);

    let mut multipart_form_data: MultipartFormData = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return json!({
                        "status": "error",
                        "message": format!("Image too large ({} MB maximum).", MAX_IMAGE_SIZE)
                    });
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return json!({
                        "status": "error",
                        "message": "The file is not an image."
                    });
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let image = multipart_form_data.raw.remove("image");

    let num_shapes: u64 = match extract_uint64(&mut multipart_form_data, "num_shapes", NUM_SHAPES_DEFAULT) {
        Some(val) => val,
        None => {
            return json!({
                "status": "error",
                "message": "num_shapes must be an unsigned integer."
            });
        }
    };
    let max_age: u64 = match extract_uint64(&mut multipart_form_data, "max_age", MAX_AGE_DEFAULT) {
        Some(val) => val,
        None => {
            return json!({
                "status": "error",
                "message": "max_age must be an unsigned integer."
            });
        }
    };
    let scale_to: u64 = match extract_uint64(&mut multipart_form_data, "scale_to", SCALE_TO_DEFAULT) {
        Some(val) => val,
        None => {
            return json!({
                "status": "error",
                "message": "scale_to must be an unsigned integer."
            });
        }
    };
    let seed: u64 = match extract_uint64(&mut multipart_form_data, "seed", SEED_DEFAULT) {
        Some(val) => val,
        None => {
            return json!({
                "status": "error",
                "message": "seed must be an unsigned integer."
            });
        }
    };

    let shape: String = match multipart_form_data.texts.remove("shape") {
        Some(mut values) => {
            let value = values.remove(0).text;
            value
        },
        None => SHAPE_DEFAULT.to_string()
    };

    match image {
        Some(mut image) => {
            let raw = image.remove(0);

            let file_name = raw.file_name.unwrap_or("image.jpg".to_string());
            let file_name_path = PathBuf::new().join(file_name.clone());
            let extension = file_name_path.extension().unwrap().to_str().unwrap();
            let data = raw.raw;

            let destination = env::temp_dir().join("primitive_web").join("input").join(request_id.clone() + "." + extension);
            let mut dest_file = fs::File::create(destination.clone()).unwrap();

            match dest_file.write_all(data.as_ref()) {
                Ok(_) => {

                    queue.inner().push(PrimitiveRequest {
                        request_id: request_id.clone(),
                        input_file_path: destination.clone(),
                        num_shapes,
                        max_age,
                        scale_to,
                        seed,
                        shape: shape.to_string()
                    });

                    json!({
                        "status": "ok",
                        "request_id": request_id.clone()
                    })
                },
                Err(_err) => json!({
                    "status": "error",
                    "message": "Internal error: unable to save image."
                })
            }

        }
        None => json!({"status": "error",
            "message": "The image field is required."
        })
    }
}

#[get("/check_status/<request_id>")]
pub fn check_status(request_id: String) -> JsonValue {
    json!({
        "request": {
            "type": "GET",
            "uri": "/check_status",
            "parameters": {"request_id": request_id}
        },
        "response": {
            "status": 501,
            "message": "not_implemented"
        }
    })
}

#[get("/get_result/<request_id>")]
pub fn get_result(request_id: String) -> JsonValue {
    json!({
        "request": {
            "type": "GET",
            "uri": "/get_result",
            "parameters": {"request_id": request_id}
        },
        "response": {
            "status": 501,
            "message": "not_implemented"
        }
    })
}

#[get("/queue_size")]
pub fn queue_size() -> JsonValue {
    json!({
        "request": {
            "type": "GET",
            "uri": "/queue_size",
            "parameters": {}
        },
        "response": {
            "status": 501,
            "message": "not_implemented"
        }
    })
}