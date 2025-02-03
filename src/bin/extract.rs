use futures::{stream, StreamExt};
use minio::s3::args::{BucketExistsArgs, MakeBucketArgs, UploadObjectArgs};
use minio::s3::client::ClientBuilder;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use pdf2image::image::DynamicImage;
use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::env;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use tempfile::Builder as TempFileBuilder;

struct XasidaPageRecord {
    name: String,
    file_name: String,
    page_index: i16,
    object_key: String,
    bucket: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments.
    let args: Vec<String> = env::args().collect();
    eprintln!("Command args: {:?}", args);
    if args.len() < 2 {
        eprintln!("Usage: {} <pdf_file_path>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];
    eprintln!("File Name: {}", file_name);

    // Read the file bytes.
    let data = std::fs::read(file_name)
        .unwrap_or_else(|_| panic!("Could not read file as bytes: {}", file_name));

    // Load PDF from bytes.
    let pdf = PDF::from_bytes(data).expect("Failed to load PDF from bytes");

    // Render all pages.
    let pages = pdf.render(
        pdf2image::Pages::All,
        RenderOptionsBuilder::default().build()?,
    )?;

    // Initialize Kafka producer.
    let producer = create_kafka_producer();

    // Initialize MinIO client and ensure the bucket exists.
    let minio_client = create_minio_client()?;
    let bucket_name = "xasida.pdf.pages";
    ensure_bucket_exists(&minio_client, bucket_name).await?;

    // Process each rendered page.
    process_pages(pages, &minio_client, bucket_name, &producer).await?;

    Ok(())
}

/// Creates and returns a configured Kafka FutureProducer.
fn create_kafka_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:19099") // Set your Kafka broker address.
        .set("message.timeout.ms", "5000")
        .set("request.timeout.ms", "5000")
        .set("retries", "3")
        .set("debug", "all")
        .create()
        .expect("Producer creation failed")
}

/// Creates and returns a MinIO client.
fn create_minio_client() -> Result<minio::s3::client::Client, Box<dyn std::error::Error>> {
    let base_url = "http://localhost:9000".parse::<BaseUrl>()?;
    let static_provider = StaticProvider::new("daaray_kamil", "daaray_kamil", None);
    let client = ClientBuilder::new(base_url)
        .provider(Some(Box::new(static_provider)))
        .build()?;
    Ok(client)
}

/// Ensures that the specified bucket exists; if not, creates it.
async fn ensure_bucket_exists(
    client: &minio::s3::client::Client,
    bucket_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let exists: bool = client
        .bucket_exists(&BucketExistsArgs::new(bucket_name)?)
        .await?;
    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(bucket_name)?)
            .await?;
    }
    Ok(())
}

/// Processes PDF pages: converts each page to grayscale, writes it to a temporary file,
/// uploads the file to MinIO, and sends a message via Kafka.
async fn process_pages(
    pages: Vec<DynamicImage>,
    minio_client: &minio::s3::client::Client,
    bucket_name: &str,
    producer: &FutureProducer,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the pages vector into a stream and then back to a Vec (optional, if you need async processing).
    let images: Vec<DynamicImage> = stream::iter(pages).collect().await;

    for image in images {
        // Create a temporary file to store the grayscale image.
        let mut temp_file = TempFileBuilder::new()
            .prefix("xasida_")
            .suffix(".jpg")
            .tempfile()?;

        // Convert image to grayscale and write as JPEG to the temp file.
        image
            .grayscale()
            .write_to(temp_file.as_file_mut(), pdf2image::image::ImageFormat::Jpeg)
            .expect("Failed to write grayscale image to temporary file");

        // Obtain the temporary file path as a string.
        let file_path = temp_file
            .path()
            .to_str()
            .expect("Temporary file path is not valid UTF-8")
            .to_owned();

        // Upload the file to MinIO.
        let upload_args = UploadObjectArgs::new(bucket_name, &file_path, &file_path)?;
        minio_client.upload_object(&upload_args).await?;

        // Send a Kafka message.
        let record = FutureRecord::to("test").key("key").payload("{test: test}");
        match producer.send(record, Duration::from_secs(3)).await {
            Ok(delivery) => println!("Message delivered: {:?}", delivery),
            Err((err, _)) => eprintln!("Error delivering message: {:?}", err),
        }
    }
    Ok(())
}
