use actix_multipart::Multipart;
use actix_web::{Error, get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::http::header::CONTENT_LENGTH;
use futures_util::TryStreamExt;
use image::DynamicImage;
use image::imageops::FilterType;
use mime::{IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG, Mime};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[get("/image/{id}")]
async fn get_image(path: web::Path<String>) -> impl Responder {
    let file_path = format!("{}/{}", "./uploads", path.into_inner());
    match std::fs::read(file_path) {
        Ok(image_content) => {
            Ok(HttpResponse::Ok()
                .content_type("image/png")
                .body(image_content))
        }
        Err(e) => {
            println!("get_image");
            Err(actix_web::Error::from(e))
        }
    }
}

#[post("/image/")]
pub async fn upload_image_handler(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };

    let max_file_count: usize = 3;
    let max_file_size: usize = 5_000_000;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let dir: &str = "./uploads/";
    if content_length > max_file_size {
        return Ok(HttpResponse::BadRequest().content_type("text/plain").body("Error upload image"));
    }

    loop {
        if current_count == max_file_count {
            break;
        }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() {
                continue;
            }
            if !legal_filetypes.contains(&filetype.unwrap()) {
                continue;
            }

            let destination: String = format!(
                "{}{}",
                dir,
                field.content_disposition().get_filename().unwrap()
            );
            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            web::block(move || async move {
                let uploaded_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).await.unwrap();
                // let save_path = format!("{}{}.png", dir, Uuid::new_v4().to_string());
                // let save_path = format!("{}{}",dir,field.content_disposition().get_filename().unwrap_or(Uuid::new_v4().to_string().as_str()));
                uploaded_img
                    .resize_exact(700, 510, FilterType::Triangle)
                    .save(&destination)
                    .unwrap();
            })
                .await
                .unwrap()
                .await;
        } else {
            break;
        }
        current_count += 1;
    }
    Ok(HttpResponse::Ok().body("Ok"))
}
