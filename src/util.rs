use futures_util::TryStreamExt;
use indicatif::{HumanBytes, HumanDuration, ProgressBar, ProgressStyle};
use reqwest::{Client, Url};
use std::path::Path;
use std::{cmp::min, fs::File, io::Write, time::Duration};

pub async fn download_file(url: Url, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    let total_size = res.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:50.cyan/blue}] {bytes:>8}/{total_bytes:>8} | {binary_bytes_per_sec:>10} | ETA: {eta_precise}")?
        .progress_chars("█►-"));

    if total_size > 0 {
        println!("Downloading {} ({})", path, HumanBytes(total_size));
    }

    let mut file = File::create(Path::new(path))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    let start_time = std::time::Instant::now();
    while let Some(chunk) = stream.try_next().await? {
        file.write_all(&chunk)?;
        downloaded = min(downloaded + (chunk.len() as u64), total_size);
        pb.set_position(downloaded);

        // Calculate and update download speed and ETA
        if downloaded > 0 {
            let elapsed = start_time.elapsed();
            let speed = downloaded as f64 / elapsed.as_secs_f64();

            if speed > 0.0 {
                let remaining_bytes = total_size.saturating_sub(downloaded);
                let eta = Duration::from_secs_f64(remaining_bytes as f64 / speed);
                pb.set_message(format!("ETA: {}", HumanDuration(eta)));
            }
        }
    }

    let total_time = start_time.elapsed();
    let avg_speed = if total_time.as_secs() > 0 {
        downloaded as f64 / total_time.as_secs_f64()
    } else {
        downloaded as f64
    };

    pb.finish_with_message(format!(
        "Download completed - {} in {} (avg speed: {}/s)",
        HumanBytes(downloaded),
        HumanDuration(total_time),
        HumanBytes(avg_speed as u64)
    ));

    Ok(())
}

