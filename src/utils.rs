use actix_files::Files;

pub fn file_handler(serve_at_path: &str, source_dir: &str) -> Files {
  Files::new(serve_at_path, source_dir)
    .show_files_listing()
    .use_last_modified(true)
}