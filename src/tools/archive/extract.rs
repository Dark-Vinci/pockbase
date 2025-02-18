use {
    std::path::Path,
    tokio::{fs, io, task},
    zip::ZipArchive,
};

pub async fn extract(src: &str, dest: &str) -> io::Result<()> {
    let file = task::spawn_blocking(move || fs::File::open(src))
        .await?
        .await?;
    let mut archive =
        task::spawn_blocking(move || ZipArchive::new(file)).await??;

    let dest_path = Path::new(dest);
    fs::create_dir_all(dest_path).await?;

    for i in 0..archive.len() {
        let mut file = task::spawn_blocking({
            let mut archive = archive.by_index(i).unwrap();
            move || archive
        })
        .await?;

        let out_path = dest_path.join(file.mangled_name());

        if file.is_dir() {
            fs::create_dir_all(&out_path).await?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).await?;
            }

            let mut outfile = fs::File::create(&out_path).await?;

            let mut buffer = Vec::new();
            task::spawn_blocking(move || std::io::copy(&mut file, &mut buffer))
                .await??;

            io::copy(&mut buffer.as_slice(), &mut outfile).await?;
        }
    }

    Ok(())
}
