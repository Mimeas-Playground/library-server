use rocket::{Error, fs::{FileServer, relative}};

#[rocket::main]
async fn main() -> Result<(), Error>{
    let _ = rocket::build()
    .mount("/", FileServer::from(relative!("web")))
    .launch().await?;

    Ok(())
}
