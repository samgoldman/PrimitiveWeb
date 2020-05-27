use rocket_contrib::json::JsonValue;
use rocket::http::ContentType;
use nanoid::nanoid;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, MultipartFormDataError};
use rocket::Data;
use std::{fs, env};
use rocket_raw_response::RawResponse;
use std::io::Write;
use std::path::PathBuf;

const VALID_SHAPES: [&str; 6] = ["TRIANGLE", "CUBIC", "QUADRATIC", "RECTANGLE", "ELLIPSE", "MIXED"];
const MAX_IMAGE_SIZE: u64 = 32; // MB

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
pub fn submit(content_type: &ContentType, data: Data) -> JsonValue {
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

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
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

    match image {
        Some(mut image) => {
            let raw = image.remove(0);

            let content_type = raw.content_type;
            let file_name = raw.file_name.unwrap_or("image.jpg".to_string());
            let file_name_path = PathBuf::new().join(file_name.clone());
            let extension = file_name_path.extension().unwrap().to_str().unwrap();
            let data = raw.raw;

            let destination = env::temp_dir().join("primitive_web").join("input").join(request_id.clone() + "." + extension);
            let mut dest_file = fs::File::create(destination).unwrap();

            match dest_file.write_all(data.as_ref()) {
                Ok(_) => json!(),
                Err(err) => json!()
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