use std::path::{Path, PathBuf};

use rocket::http::ContentType;
use rocket::response::content::Content;
use rocket::response::NamedFile;
use rocket::Route;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::util::Cached;
use crate::error::Error;
use crate::CONFIG;

pub fn routes() -> Vec<Route> {
    if CONFIG.web_vault_enabled() {
        routes![web_index, app_id, web_files, attachments, alive, static_files]
    } else {
        routes![attachments, alive, static_files]
    }
}

#[get("/")]
fn web_index() -> Cached<Option<NamedFile>> {
    Cached::short(NamedFile::open(
        Path::new(&CONFIG.web_vault_folder()).join("index.html"),
    ).ok())
}

#[get("/app-id.json")]
fn app_id() -> Cached<Content<Json<Value>>> {
    let content_type = ContentType::new("application", "fido.trusted-apps+json");

    Cached::long(Content(
        content_type,
        Json(json!({
        "trustedFacets": [
            {
            "version": { "major": 1, "minor": 0 },
            "ids": [
                &CONFIG.domain(),
                "ios:bundle-id:com.8bit.bitwarden",
                "android:apk-key-hash:dUGFzUzf3lmHSLBDBIv+WaFyZMI" ]
            }]
        })),
    ))
}

#[get("/<p..>", rank = 10)] // Only match this if the other routes don't match
fn web_files(p: PathBuf) -> Cached<Option<NamedFile>> {
    Cached::long(NamedFile::open(Path::new(&CONFIG.web_vault_folder()).join(p)).ok())
}

#[get("/attachments/<uuid>/<file..>")]
fn attachments(uuid: String, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&CONFIG.attachments_folder()).join(uuid).join(file)).ok()
}

#[get("/alive")]
fn alive() -> Json<String> {
    use crate::util::format_date;
    use chrono::Utc;

    Json(format_date(&Utc::now().naive_utc()))
}

#[get("/bwrs_static/<filename>")]
fn static_files(filename: String) -> Result<Content<&'static [u8]>, Error> {
    match filename.as_ref() {
        "mail-github.png" => Ok(Content(ContentType::PNG, include_bytes!("../static/images/mail-github.png"))),
        "logo-gray.png" => Ok(Content(ContentType::PNG, include_bytes!("../static/images/logo-gray.png"))),
        "error-x.svg" => Ok(Content(ContentType::SVG, include_bytes!("../static/images/error-x.svg"))),

        "bootstrap.css" => Ok(Content(ContentType::CSS, include_bytes!("../static/scripts/bootstrap.css"))),
        "bootstrap-native-v4.js" => Ok(Content(ContentType::JavaScript, include_bytes!("../static/scripts/bootstrap-native-v4.js"))),
        "md5.js" => Ok(Content(ContentType::JavaScript, include_bytes!("../static/scripts/md5.js"))),
        "identicon.js" => Ok(Content(ContentType::JavaScript, include_bytes!("../static/scripts/identicon.js"))),
        _ => err!("Image not found"),
    }
}