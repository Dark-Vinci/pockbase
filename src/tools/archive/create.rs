use {
    std::{
        ops::{Deref, DerefMut},
        path::Path,
        sync::Arc,
    },
    tokio::{fs, io, sync::Mutex, task},
    zip::{write::FileOptions, ZipWriter},
};

pub async fn create(
    src: &str,
    dst: &str,
    skip_path: &[&str],
) -> io::Result<()> {
    let destination_path = Path::new(src);

    if let Some(parent) = destination_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let file = fs::File::create(destination_path).await?;
    let writer = Arc::new(Mutex::new(ZipWriter::new(file)));

    zip_add_fs(writer.clone(), Path::new(dst), skip_path).await?;

    let writer_clone = writer.clone();
    task::spawn_blocking(move || {
        writer_clone.lock().unwrap().finish().unwrap() // needs attention
    })
    .await?;

    Ok(())
}

async fn zip_add_fs(
    writer: Arc<Mutex<ZipWriter<fs::File>>>,
    src: &Path,
    skip_paths: &[&str],
) -> io::Result<()> {
    if src.is_dir() {
        let mut entries = fs::read_dir(src).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let relative_path = path.strip_prefix(src).unwrap();

            if skip_paths
                .iter()
                .any(|&skip| relative_path.starts_with(skip))
            {
                continue;
            }

            if path.is_dir() {
                continue;
            }

            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            writer
                .lock()
                .await
                .start_file(relative_path.to_string_lossy(), options)?;

            let mut file = fs::File::open(&path).await?;

            let mut writer_guard = writer.lock().await;

            task::spawn_blocking(move || {
                io::copy(&mut file, &mut *writer_guard).unwrap()
            })
            .await?;
        }
    }

    Ok(())
}
