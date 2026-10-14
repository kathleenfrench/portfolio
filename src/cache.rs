// cache-control policies for static assets
//
// webpack puts a content hash in the js/wasm filenames, so those can be cached
// forever: a new build means a new filename. everything else keeps a stable
// name, so it gets a short ttl and browsers/cloudflare revalidate via etag.

const IMMUTABLE: &str = "public, max-age=31536000, immutable";
const IMAGES: &str = "public, max-age=86400";
const DEFAULT: &str = "public, max-age=3600";

pub fn policy_for(path: &str) -> Option<&'static str> {
    if !path.starts_with("/assets/") {
        return None;
    }

    if path.starts_with("/assets/js/") && is_content_hashed(path) {
        Some(IMMUTABLE)
    } else if path.starts_with("/assets/images/") {
        Some(IMAGES)
    } else {
        Some(DEFAULT)
    }
}

// matches any 8+ hex char segment before the extension, e.g.
// `main.3f9a1c2e.js` or `c706ea3e8f23bc1bc0d7.module.wasm`
fn is_content_hashed(path: &str) -> bool {
    let file = path.rsplit('/').next().unwrap_or_default();
    let parts: Vec<&str> = file.split('.').collect();

    parts.len() >= 2
        && parts[..parts.len() - 1]
            .iter()
            .any(|p| p.len() >= 8 && p.chars().all(|c| c.is_ascii_hexdigit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashed_bundles_are_immutable() {
        assert_eq!(policy_for("/assets/js/main.3f9a1c2e.js"), Some(IMMUTABLE));
        assert_eq!(
            policy_for("/assets/js/c706ea3e8f23bc1bc0d7.module.wasm"),
            Some(IMMUTABLE)
        );
    }

    #[test]
    fn unhashed_assets_get_short_ttl() {
        assert_eq!(policy_for("/assets/js/treasure.js"), Some(DEFAULT));
        assert_eq!(policy_for("/assets/css/main.css"), Some(DEFAULT));
        assert_eq!(policy_for("/assets/images/kf.jpg"), Some(IMAGES));
    }

    #[test]
    fn pages_are_not_cached() {
        assert_eq!(policy_for("/"), None);
        assert_eq!(policy_for("/dr9lrf26db8ori9"), None);
    }
}
