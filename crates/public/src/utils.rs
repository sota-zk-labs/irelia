use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use base64::Engine;

pub fn save_cairo_pie(encoded_pie: &String, file_name: &str) -> anyhow::Result<PathBuf> {
    let path = pie_storage_path(file_name);

    fs::create_dir_all(path.parent().unwrap())?;
    let decoded_pie = base64::engine::general_purpose::STANDARD.decode(encoded_pie)?;
    let reader = std::io::Cursor::new(decoded_pie);
    File::create(&path)?.write_all(reader.get_ref())?;

    check_cairo_pie_zip(file_name)?;
    Ok(path)
}

fn pie_storage_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cairo_pies")
        .join(format!("{}.zip", file_name))
}

fn check_cairo_pie_zip(file_name: &str) -> anyhow::Result<()> {
    let path = pie_storage_path(file_name);
    let mut zip_archive = zip::ZipArchive::new(File::open(path)?)?;
    for file in [
        "additional_data.json",
        "execution_resources.json",
        "memory.bin",
        "metadata.json",
        "version.json",
    ] {
        zip_archive.by_name(file)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::fs::File;
    use std::io::Read;

    use base64::engine::general_purpose;
    use base64::Engine;

    use crate::utils::save_cairo_pie;

    #[test]
    fn test_save_cairo_pie() {
        let encoded_origin = encode_zip("./src/assets/test_data/fibonacci_with_output.zip");

        let written_path = save_cairo_pie(&encoded_origin, "test").unwrap();
        let encoded_written = encode_zip(written_path.to_str().unwrap());

        assert_eq!(encoded_origin, encoded_written);

        let cairo_pie = fs::read_to_string("./src/assets/test_data/encoded_cairo_pie.txt").unwrap();
        save_cairo_pie(&cairo_pie, "test2").unwrap();
    }

    fn encode_zip(path: &str) -> String {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        general_purpose::STANDARD.encode(&buffer)
    }
}
