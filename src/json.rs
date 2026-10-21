use std::{fs, path::Path};

pub fn load_json<P>(path: &P) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)?;

    Ok(content)
}
