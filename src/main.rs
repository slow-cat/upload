use std::env;
use actix_multipart::Multipart;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use futures_util::stream::StreamExt as _;
use std::fs::File;
use std::io::Write;
use sanitize_filename::sanitize;
use qrcode::QrCode;
use qrcode::types::Color;
async fn upload_form() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <html><body>
        <h1>Upload audio file</h1>
        <form target="/" method="post" enctype="multipart/form-data">
        <input type="file" name="file"><br>
        <input type="submit" value="Upload">
        </form>
        </body></html>
    "#,
    )
}


async fn upload_file(mut payload: Multipart) -> impl Responder {
    while let Some(Ok(mut field)) = payload.next().await {
        let content_disposition = field.content_disposition();

        let filename = match content_disposition.get_filename() {
            Some(name) => name,
            None => return HttpResponse::BadRequest().body("Filename missing"),
        };

        let filepath = format!("./{}", sanitize(filename));
        let mut f = File::create(filepath).unwrap();

        while let Some(Ok(chunk)) = field.next().await {
            f.write_all(&chunk).unwrap();
        }
    }
    HttpResponse::Ok().body("Succeeded")
}
fn QR(text: &str) {
    let code = QrCode::new(text).unwrap();

    let width = code.width();

    for y in 0..width {
        for x in 0..width {
            let color = code[(x, y)];
            if color == Color::Dark {
                print!("██");  // black
            } else {
                print!("  ");  // white
            }
        }
        println!();
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();


    let bind_ip = args.get(1).map_or("localhost", |v| v.as_str());
    let bind_port = args.get(2).and_then(|p| p.parse::<u16>().ok()).unwrap_or(8000);

    let url = format!("http://{}:{}", bind_ip, bind_port);
    println!("{}", url);
    QR(&url);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(upload_form))
            .route("/", web::post().to(upload_file))
    })
    .bind((bind_ip, bind_port))?
    .run()
    .await
}

