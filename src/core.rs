//! 生成短链接核心包

use fasthash::murmur3;

/// 获取URL的哈希值
fn get_hash(url: &str) -> u32 {
    murmur3::hash32(url.as_bytes())
}

/// 通过指定的种子获取URL的哈希值
fn get_hash_with_seed(url: &str, seed: u32) -> u32 {
    murmur3::hash32_with_seed(url.as_bytes(), seed)
}

/// 将u32类型转成base62
fn u32_to_62(hash: u32) -> String {
    let dict = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut n = hash;
    let mut chars: Vec<char> = vec![];
    while n > 0 {
        let i = (n % 62) as usize;
        let c = dict.chars().nth(i).unwrap();
        chars.push(c);
        n /= 62;
    }
    chars.reverse();
    chars.into_iter().collect::<String>()
}

/// 获取URL对应的短链接
pub fn short_url(url: &str) -> String {
    let hash = get_hash(url);
    u32_to_62(hash)
}

/// 通过指定的种子获取URL对应的短链接
pub fn short_url_with_seed(url: &str, seed: u32) -> String {
    let hash = get_hash_with_seed(url, seed);
    u32_to_62(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash() {
        assert_eq!(get_hash("https://axum.rs"), 3506573287);
    }

    #[test]
    fn test_get_hash_with_seed() {
        assert_eq!(
            get_hash_with_seed("https://axum.rs", 0),
            get_hash("https://axum.rs")
        );
        assert_eq!(get_hash_with_seed("https://axum.rs", 0), 3506573287);
        assert_eq!(get_hash_with_seed("https://axum.rs", 100), 888869650);
    }

    #[test]
    fn test_short_url() {
        assert_eq!(short_url("https://axum.rs"), "3PjdTF".to_string());
    }
    #[test]
    fn test_short_url_with_seed() {
        assert_eq!(
            short_url_with_seed("https://axum.rs", 0),
            short_url("https://axum.rs")
        );
        assert_eq!(
            short_url_with_seed("https://axum.rs", 0),
            "3PjdTF".to_string()
        );
        assert_eq!(
            short_url_with_seed("https://axum.rs", 100),
            "Y9BBg".to_string()
        );
    }
}
