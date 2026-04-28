use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures_util::StreamExt;
use uuid::Uuid;
use std::path::PathBuf;
use tokio::fs;
use log::info;

use crate::middleware::auth::AuthenticatedUser;

const UPLOAD_DIR: &str = "uploads";

pub async fn upload_image(
    _user: AuthenticatedUser,
    mut payload: Multipart,
) -> HttpResponse {
    let upload_dir = PathBuf::from(UPLOAD_DIR);
    if !upload_dir.exists() {
        if let Err(e) = fs::create_dir_all(&upload_dir).await {
            log::error!("Failed to create upload dir: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "服务器错误"
            }));
        }
    }

    let mut uploaded_urls: Vec<String> = Vec::new();

    while let Some(field) = payload.next().await {
        match field {
            Ok(mut field) => {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("image.png")
                    .to_string();

                let ext = std::path::Path::new(&filename)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("png")
                    .to_lowercase();

                let allowed_exts = ["png", "jpg", "jpeg", "gif", "webp", "bmp"];
                if !allowed_exts.contains(&ext.as_str()) {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "不支持的文件格式，仅支持 png, jpg, jpeg, gif, webp, bmp"
                    }));
                }

                let file_id = Uuid::new_v4().to_string();
                let save_filename = format!("{}.{}", file_id, ext);
                let file_path = upload_dir.join(&save_filename);

                let mut file_data: Vec<u8> = Vec::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => file_data.extend_from_slice(&data),
                        Err(e) => {
                            log::error!("Error reading chunk: {:?}", e);
                            return HttpResponse::BadRequest().json(serde_json::json!({
                                "error": "读取文件失败"
                            }));
                        }
                    }
                }

                if file_data.len() > 10 * 1024 * 1024 {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "文件大小超过限制（最大10MB）"
                    }));
                }

                match fs::write(&file_path, &file_data).await {
                    Ok(_) => {
                        let file_url = format!("/uploads/{}", save_filename);
                        uploaded_urls.push(file_url);
                        info!("Uploaded file: {} ({} bytes)", save_filename, file_data.len());
                    }
                    Err(e) => {
                        log::error!("Failed to save file: {:?}", e);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "保存文件失败"
                        }));
                    }
                }
            }
            Err(e) => {
                log::error!("Error reading multipart field: {:?}", e);
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "读取上传数据失败"
                }));
            }
        }
    }

    if uploaded_urls.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "没有上传任何文件"
        }));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "urls": uploaded_urls,
        "message": "上传成功"
    }))
}
