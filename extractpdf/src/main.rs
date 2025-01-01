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
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), PDF2ImageError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    // Get the URL from the command-line arguments
    let file_name = &args[1];

    let pdf = PDF::from_file(file_name).unwrap();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=8),
        RenderOptionsBuilder::default().build()?,
    );

    match pages {
        Ok(p) => {
            println!("Found some pages");
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", "https://localhost:19099") // Set your Kafka broker address
                .create()
                .expect("Producer creation failed");

                let base_url = "http://localhost:9000".parse::<BaseUrl>().unwrap();
                let static_provider = StaticProvider::new("miniominio", "miniominio", None);

                let client = ClientBuilder::new(base_url.clone())
                    .provider(Some(Box::new(static_provider)))
                    .build()
                    .unwrap();

                let bucket_name: &str = "xasida.pdf.pages";
                let exists: bool = client
                    .bucket_exists(&BucketExistsArgs::new(bucket_name).unwrap())
                    .await
                    .unwrap();

                // Make 'bucket_name' bucket if not exist.
                if !exists {
                    client
                        .make_bucket(&MakeBucketArgs::new(bucket_name).unwrap())
                        .await
                        .unwrap();
                }
            let items = p.to_vec();
            let streamed: Vec<DynamicImage> = stream::iter(items).collect().await;
            for d in streamed {
                // Name of the object that will be stored in the bucket
                let object_name: &str = "asiaphotos-2015.pdf";

                //info!("filename {}", &filename.to_str().unwrap());

                client
                    .upload_object(
                        &mut UploadObjectArgs::new(
                            &bucket_name, 
                            object_name, 
                            "/Users/ppathe/rust_projects/xasida/tmp/Yawma%20Arbuhaan%20i%20rajab.pdf"
                        ).unwrap(),
                    )
                    .await
                    .unwrap();

               // match producer
               //     .send(
               //         FutureRecord::to("test").key("key").payload("{test: test}"),
               //         Duration::from_secs(0),
               //     )
               //     .await
               // {
               //     Ok(delivery) => println!("Message delivered: {:?}", delivery),
               //     Err((err, _)) => eprintln!("Error delivering message: {}", err),
               // }
            }
        }
        Err(e) => println!("Error {}", e),
    }

    Ok(())
}
