use actix_files::NamedFile;

use crate::CONFIG;

pub async fn favicon() -> std::io::Result<NamedFile> {
    NamedFile::open(&CONFIG.static_paths.favicon)
}
