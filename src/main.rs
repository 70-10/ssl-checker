use clap::Parser;
use openssl::ssl::{SslConnector, SslMethod};
use openssl::x509::X509;
use serde::Serialize;
use std::error::Error;
use std::net::TcpStream;

#[derive(Serialize)]
struct CertificateInfo {
    subject: Vec<NameEntry>,
    issuer: Vec<NameEntry>,
    validity: Validity,
    public_key: PublicKeyInfo,
    version: i32,
    serial_number: String,
    signature_algorithm: String,
}

#[derive(Serialize)]
struct NameEntry {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct Validity {
    not_before: String,
    not_after: String,
}

#[derive(Serialize)]
struct PublicKeyInfo {
    key_type: String,
    key_size: Option<u32>,
    modulus: Option<String>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Domain name to check SSL certificate
    domain: String,

    /// Output in JSON format
    #[arg(long)]
    json: bool,
}

fn get_certificate(domain: &str) -> Result<X509, Box<dyn Error>> {
    let addr = format!("{}:443", domain);
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = builder.build();

    let stream = TcpStream::connect(&addr)?;
    let ssl_stream = connector.connect(domain, stream)?;
    let ssl = ssl_stream.ssl();

    ssl.peer_certificate()
        .ok_or_else(|| "Failed to get certificate".into())
}

fn extract_name_entries(
    name: &openssl::x509::X509NameRef,
) -> Result<Vec<NameEntry>, Box<dyn Error>> {
    let mut entries = Vec::new();
    for entry in name.entries() {
        let obj = entry.object();
        let nid = obj.nid();
        entries.push(NameEntry {
            key: nid.short_name()?.to_string(),
            value: entry.data().as_utf8()?.to_string(),
        });
    }
    Ok(entries)
}

fn get_certificate_info(cert: &X509) -> Result<CertificateInfo, Box<dyn Error>> {
    let subject = extract_name_entries(cert.subject_name())?;
    let issuer = extract_name_entries(cert.issuer_name())?;

    let public_key = cert.public_key()?;
    let mut key_info = PublicKeyInfo {
        key_type: "Unknown".to_string(),
        key_size: None,
        modulus: None,
    };

    if let Ok(rsa) = public_key.rsa() {
        key_info = PublicKeyInfo {
            key_type: "RSA".to_string(),
            key_size: Some(rsa.size() * 8),
            modulus: Some(rsa.n().to_hex_str()?.to_string()),
        };
    }

    Ok(CertificateInfo {
        subject,
        issuer,
        validity: Validity {
            not_before: cert.not_before().to_string(),
            not_after: cert.not_after().to_string(),
        },
        public_key: key_info,
        version: cert.version(),
        serial_number: cert.serial_number().to_bn()?.to_hex_str()?.to_string(),
        signature_algorithm: cert.signature_algorithm().object().to_string(),
    })
}

fn print_text_output(info: &CertificateInfo) {
    println!("--- Certificate Information ---");
    println!("Subject:");
    for entry in &info.subject {
        println!("  {} = {}", entry.key, entry.value);
    }
    println!("Issuer:");
    for entry in &info.issuer {
        println!("  {} = {}", entry.key, entry.value);
    }
    println!("Validity Period:");
    println!("  Not Before: {}", info.validity.not_before);
    println!("  Not After:  {}", info.validity.not_after);
    println!("Version: {}", info.version);
    println!("Serial Number: {}", info.serial_number);
    println!("Signature Algorithm: {}", info.signature_algorithm);

    if info.public_key.key_type == "RSA" {
        println!("Public Key Type: {}", info.public_key.key_type);
        if let Some(key_size) = info.public_key.key_size {
            println!("RSA Key Size: {} bits", key_size);
        }
        if let Some(modulus) = &info.public_key.modulus {
            println!("RSA Modulus: {}", modulus);
        }
    } else {
        println!("Public Key Type: {}", info.public_key.key_type);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cert = get_certificate(&args.domain)?;
    let info = get_certificate_info(&cert)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&info)?);
    } else {
        print_text_output(&info);
    }

    Ok(())
}
