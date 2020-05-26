use rocket_contrib::json::JsonValue;
use rocket::http::ContentType;
use nanoid::nanoid;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, MultipartFormDataError};
use rocket::Data;
use std::{fs, env};
use rocket_raw_response::RawResponse;
use std::io::Write;
use std::path::PathBuf;

#[post("/submit", data = "<data>")]
pub fn submit(content_type: &ContentType, data: Data) -> Result<RawResponse, &'static str> {
    let request_id = nanoid!(42);

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err("The file is too large.");
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err("The file is not an image.");
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
            let _save_result = dest_file.write_all(data.as_ref());

            Ok(RawResponse::from_vec(data, Some(file_name.clone()), content_type))
        }
        None => Err("Please input a file."),
    }
}

#[get("/is_ready/<request_id>")]
pub fn is_ready(request_id: String) -> JsonValue {
    json!({
        "request": {
            "type": "GET",
            "uri": "/is_ready",
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