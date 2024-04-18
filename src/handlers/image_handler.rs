use crate::schema::response_schema::ImageOptions;
use actix_multipart::Multipart;
use actix_web::dev::ResourcePath;
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use futures_util::TryStreamExt;
use image::imageops::FilterType;
use image::DynamicImage;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub fn image_scope() -> actix_web::Scope {
    web::scope("/images")
        .route("/{image_path:.*}", web::get().to(get_image))
        .route("", web::post().to(upload_images_handler))
        .route("/image", web::post().to(upload_image_handler))
}

async fn get_image(image_path: web::Path<String>) -> impl Responder {
    let file_path = format!("{}{}", "./uploads/", image_path.into_inner());
    match std::fs::read(file_path) {
        Ok(image_content) => Ok(HttpResponse::Ok()
            .content_type("image/png")
            .body(image_content)),
        Err(e) => {
            println!("get_image");
            Err(actix_web::Error::from(e))
        }
    }
}

pub async fn upload_images_handler(
    directory: web::Query<ImageOptions>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };
    let max_file_count: usize = 3;
    let max_file_size: usize = 5_000_000;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let dir: &str = "./uploads/";
    let order_dir = format!("{}{}/", dir, directory.dir);
    if !Path::new(order_dir.path()).exists() {
        fs::create_dir(order_dir.path()).await?;
    }
    let mut images_path: Vec<String> = Vec::new();
    if content_length > max_file_size {
        return Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("Error upload image"));
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
            let file_path = format!(
                "{}/{}",
                directory.dir,
                field.content_disposition().get_filename().unwrap()
            );
            let destination: String = format!(
                "{}{}",
                order_dir,
                field.content_disposition().get_filename().unwrap()
            );
            images_path.insert(0, file_path.clone());
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
                    .resize_exact(640, 380, FilterType::Triangle)
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
    Ok(HttpResponse::Ok().json(images_path))
}

pub async fn upload_image_handler(
    directory: web::Query<ImageOptions>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };
    let max_file_size: usize = 20_000_000;
    let dir: &str = "./uploads/";
    let order_dir = format!("{}{}/", dir, directory.dir);
    if !Path::new(order_dir.path()).exists() {
        fs::create_dir(order_dir.path()).await?;
    }
    let mut images_path: String = String::new();
    if content_length > max_file_size {
        return Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("Error upload image"));
    }
    if let Ok(Some(mut field)) = payload.try_next().await {
        let file_path = format!(
            "{}/{}",
            directory.dir,
            field.content_disposition().get_filename().unwrap()
        );
        let destination: String = format!(
            "{}{}",
            order_dir,
            field.content_disposition().get_filename().unwrap()
        );
        images_path = file_path.clone();
        let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
        while let Ok(Some(chunk)) = field.try_next().await {
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }

        web::block(move || async move {
            let uploaded_img: DynamicImage = image::open(&destination).unwrap();
            let _ = fs::remove_file(&destination).await.unwrap();
            uploaded_img
                .resize_exact(640, 380, FilterType::Triangle)
                .save(&destination)
                .unwrap();
        })
        .await
        .unwrap()
        .await;
    }
    Ok(HttpResponse::Ok().json(images_path))
}
