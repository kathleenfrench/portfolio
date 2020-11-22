use actix_files::NamedFile;

pub async fn favicon() -> Result<NamedFile, std::io::Error> {
    Ok(NamedFile::open("static/favicon.ico")?)
}