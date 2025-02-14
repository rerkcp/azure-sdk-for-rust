/*
Creates a 2048-bit RSA key expiring in 90 days using the data plane APIs
Assumes you already have a keyvault instance created.

Use the following command from the ./services folder, where <YourKeyVaultName> is the
name of the keyvault you want the key created in, and <KeyName> is the name of the key
you want to create.

cargo run --package azure_svc_keyvault --example create_key <YourKeyVaultName> <KeyName>
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_svc_keyvault::models::{key_create_parameters::Kty, Attributes, KeyAttributes, KeyCreateParameters};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential {});
    let keyvault_name = std::env::args().nth(1).expect("please specify an existing keyvault");
    let key_name = std::env::args().nth(2).expect("please specify the name of the key to create");
    let endpoint = format!("https://{}.vault.azure.net", keyvault_name);
    let scopes = &["https://vault.azure.net"];
    let client = azure_svc_keyvault::ClientBuilder::new(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    // Configure the not-before (nbf) and expiration (exp) dates
    let nbf = chrono::Utc::now();
    let exp = nbf + chrono::Duration::days(90);

    let mut key_attributes = KeyAttributes::new();
    key_attributes.attributes = Attributes::new();
    key_attributes.attributes.nbf = Some(nbf.timestamp());
    key_attributes.attributes.exp = Some(exp.timestamp());

    // Configure key type and size
    let mut key_create_params = KeyCreateParameters::new(Kty::Rsa);
    key_create_params.key_size = Some(2048);
    key_create_params.attributes = Some(key_attributes);

    println!("Creating key '{}' in key vault '{}'.", key_name, keyvault_name);
    client.create_key(&key_name, key_create_params).into_future().await?;
    println!("Key '{}' created!", key_name);

    Ok(())
}
