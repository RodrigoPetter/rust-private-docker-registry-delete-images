pub fn byte_to_mega(bytes: &usize) -> f64 {
    return (bytes.clone() as f64 / 1024.0) / 1024.0;
}

pub fn mega_to_giga(megas: &f64) -> f64 {
    return megas / 1024.0;
}

pub fn format_size(size: &f64) -> String {
    if size.clone() < 1000.0 {
        return format!("{:<7.2}MB", size);
    } else {
        return format!("{:<7.2}GB", mega_to_giga(size));
    }
}