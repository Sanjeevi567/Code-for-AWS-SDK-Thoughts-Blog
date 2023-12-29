use std::{io::Read, time::SystemTime};

use aws_config::{load_defaults, load_from_env, BehaviorVersion, Region, SdkConfig};
use aws_credential_types::{
    credential_fn::provide_credentials_fn,
    provider::{ProvideCredentials, SharedCredentialsProvider},
    Credentials,
};

use aws_sdk_sesv2::{
    types::{Destination, EmailContent, Template},
    Client as SesV2Client,
};

#[tokio::main]
async fn main() {
    //method 1
    //let sdk_config = load_from_env().await;

    //method 2
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;

    let credentials = sdk_config
        .credentials_provider()
        .unwrap()
        .provide_credentials()
        .await;
    match credentials {
        Ok(credentials) => {
            println!("Access Key ID: {}", credentials.access_key_id());
            println!("Secret Access key ID: {}", credentials.secret_access_key());
            println!(
                "Session token if any: {}",
                credentials.session_token().unwrap_or_default()
            );
        }
        Err(error) => println!("{}", error.to_string()),
    }
    println!("Region: {:?}", sdk_config.region());

    // Method 3: Not recommended
    // Here, we are hardcoding credentials into the application instead of fetching them from the system.
    // Do not use this in production or as a library.
    let build_credentials_provider = Credentials::new(
        "your_access_key_id",
        "your_secret_key_id",
        Some("your_session_token".into()),
        Some(SystemTime::UNIX_EPOCH),
        "admin",
    );

    let sdk_config_builder = SdkConfig::builder()
        .region(Some(Region::new("ap-south-1")))
        .credentials_provider(SharedCredentialsProvider::new(build_credentials_provider))
        .build();

    //Crate Version
    println!("{}", aws_sdk_sesv2::meta::PKG_VERSION);

    date_time_format();
}

async fn send_simple_email(recipient_email: &str) {
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;
    let ses_client = SesV2Client::new(&sdk_config);

    let destination_emails = Destination::builder().to_addresses(recipient_email).build();

    let template = Template::builder()
        .template_name("your_template_name")
        .template_data("A json of key-value pair for the choosen template above")
        .build();

    let email_content = EmailContent::builder().template(template).build();
    ses_client
        .send_email()
        .from_email_address("Your_verified_from_email_address")
        .destination(destination_emails)
        .content(email_content)
        .send()
        .await
        .expect("Error while sending emails");
}

fn date_time_format() {
    use aws_sdk_s3::primitives::{DateTime, DateTimeFormat};

    let seconds = DateTime::from_secs(1209034013);
    let format_seconds_to_http_date = seconds.fmt(DateTimeFormat::HttpDate).unwrap();
    let format_seconds_to_date_time = seconds.fmt(DateTimeFormat::DateTime).unwrap();
    println!("{}", format_seconds_to_http_date);
    println!("{}", format_seconds_to_date_time);
}

async fn detect_faces() {
    use aws_sdk_rekognition::{types::Attribute, Client as RekogClient};
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;
    let rekognition_client = RekogClient::new(&sdk_config);

    let collection_of_attributes = vec![
        Attribute::Gender,
        Attribute::EyesOpen,
        Attribute::Smile,
        Attribute::Sunglasses,
    ];

    rekognition_client
        .detect_faces()
        .attributes(Attribute::Gender)
        .set_attributes(Some(collection_of_attributes))
        .send()
        .await
        .unwrap();
}

async fn public_field_access() {
    use aws_sdk_s3::Client as S3Client;
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;
    let s3_client = S3Client::new(&sdk_config);

    let get_object_output = s3_client
        .get_object()
        .bucket("bucket_name")
        .key("Key object name")
        .send()
        .await
        .expect("Error while getting object from s3");
    //let get_data = get_object_output.body().collect();
    //let bytes = get_object_output.body.collect();
}

async fn upload_file_using_bytestream(file_path: &str) {
    use aws_sdk_s3::{primitives::ByteStream, Client as S3Client};
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;
    let s3_client = S3Client::new(&sdk_config);

    let byte_stream = ByteStream::from_path(file_path)
        .await
        .expect("Error while opening file path");

    s3_client
        .put_object()
        .bucket("Your_bucket_name")
        .key("use your image name as your key name")
        .body(byte_stream)
        .send()
        .await
        .expect("Error while uploading onject to s3 bucket");
}

async fn upload_image_using_blob(image_path: &str) {
    use aws_sdk_rekognition::{primitives::Blob, types::Image, Client as RekogClient};
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;
    let rekognition_client = RekogClient::new(&sdk_config);

    let mut read_content = std::fs::File::open(image_path).expect("Error while opening image path");
    let mut read_into_vector = Vec::new();
    read_content.read_to_end(&mut read_into_vector).unwrap();

    let blob_type = Blob::new(read_into_vector);

    let build_image = Image::builder().bytes(blob_type).build();

    rekognition_client
        .detect_faces()
        .image(build_image)
        .send()
        .await
        .expect("Error while detecting face");
}

async fn using_static_slices() {
    //let access_key_id = "";
    static access_key_id: &str = "";
    //let secret_access_key = "";
    static secret_access_key: &str = "";

    let provider_credentials_fn = provide_credentials_fn(|| async {
        Ok(Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "admin",
        ))
    });
}

async fn using_owned_type() {
    let access_key_id = String::new();
    let secret_access_key = String::new();

    let provider_credentials_fn = provide_credentials_fn(|| async {
        Ok(Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "admin",
        ))
    });
}

async fn using_borrowed_type() {
    let access_key_id = "";
    let secret_access_key = "";

    let provider_credentials_fn = provide_credentials_fn(|| async {
        Ok(Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "admin",
        ))
    });
}

async fn using_non_sentable_type() {
    let mut access_key_id = std::rc::Rc::new(String::new());
    let mut secret_access_key = String::new();

    let provider_credentials_fn = provide_credentials_fn(|| async {
        access_key_id;
        Ok(Credentials::new("", "", None, None, "admin"))
    });
}
