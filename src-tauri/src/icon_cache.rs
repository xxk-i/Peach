pub struct IconCache {
    
}

#[cfg(target_os = "macos")]
fn init_macos() -> IconCache {
    IconCache {}
}

pub fn initialize() -> IconCache {
    if cfg!(target_os = "macos") {
        return init_macos();
    }

    IconCache {}
}