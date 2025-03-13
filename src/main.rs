use clap::Parser;
use openssl::ssl::{SslConnector, SslMethod};
use std::error::Error;
use std::net::TcpStream;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Domain name to check SSL certificate
    #[arg(short, long)]
    domain: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let domain = args.domain;
    let addr = format!("{}:443", domain);

    // Create SslConnector (using TLS)
    let connector = SslConnector::builder(SslMethod::tls())?.build();

    // Establish TCP connection and TLS connection
    let stream = TcpStream::connect(&addr)?;
    let ssl_stream = connector.connect(&domain, stream)?;
    let ssl = ssl_stream.ssl();

    // Get the certificate presented by the server
    let cert = ssl.peer_certificate().ok_or("Failed to get certificate")?;

    // Display certificate information
    println!("--- Certificate Information ---");
    // subject_name() and issuer_name() don't implement Display, so output each entry
    println!("Subject:");
    for entry in cert.subject_name().entries() {
        let obj = entry.object();
        let nid = obj.nid();
        let key = nid.short_name().unwrap_or("unknown");
        let data = entry.data().as_utf8()?;
        println!("  {} = {}", key, data);
    }
    println!("Issuer:");
    for entry in cert.issuer_name().entries() {
        let obj = entry.object();
        let nid = obj.nid();
        let key = nid.short_name().unwrap_or("unknown");
        let data = entry.data().as_utf8()?;
        println!("  {} = {}", key, data);
    }
    println!("Validity Period:");
    println!("  Not Before: {}", cert.not_before().to_string());
    println!("  Not After:  {}", cert.not_after().to_string());

    // Get public key (cryptographic information)
    let public_key = cert.public_key()?;
    // Display public key type and details (for RSA)
    if let Ok(rsa) = public_key.rsa() {
        let key_size = rsa.size() * 8;
        println!("Public Key Type: RSA");
        println!("RSA Key Size: {} bits", key_size);
        // Display modulus in hexadecimal (if needed)
        let modulus = rsa.n().to_hex_str()?;
        println!("RSA Modulus: {}", modulus);
    } else {
        println!("Public key is not RSA.");
    }

    Ok(())
}
