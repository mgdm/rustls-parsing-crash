use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
use std::{env, error::Error};

const EMBEDDED_CERT: &'static str = r"MIIDcTCCAlmgAwIBAgIVAOYJ/nrqAGiM4CS07SAbH+9StETRMA0GCSqGSIb3DQEB
BQUAMFAxCzAJBgNVBAYTAlBMMSgwJgYDVQQKDB9LcmFqb3dhIEl6YmEgUm96bGlj
emVuaW93YSBTLkEuMRcwFQYDVQQDDA5TWkFGSVIgUk9PVCBDQTAeFw0xMTEyMDYx
MTEwNTdaFw0zMTEyMDYxMTEwNTdaMFAxCzAJBgNVBAYTAlBMMSgwJgYDVQQKDB9L
cmFqb3dhIEl6YmEgUm96bGljemVuaW93YSBTLkEuMRcwFQYDVQQDDA5TWkFGSVIg
Uk9PVCBDQTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKxHL49ZMTml
6g3wpYwrvQKkvc0Kc6oJ5sxfgmp1qZfluwbv88BdocHSiXlY8NzrVYzuWBp7J/9K
ULMAoWoTIzOQ6C9TNm4YbA9A1jdX1wYNL5Akylf8W5L/I4BXhT9KnlI6x+a7BVAm
nr/Ttl+utT/Asms2fRfEsF2vZPMxH4UFqOAhFjxTkmJWf2Cu4nvRQJHcttB+cEAo
ag/hERt/+tzo4URz6x6r19toYmxx4FjjBkUhWQw1X21re//Hof2+0YgiwYT84zLb
eqDqCOMOXxvH480yGDkh/QoazWX3U75HQExT/iJlwnu7I1V6HXztKIwCBjsxffbH
3jOshCJtywcCAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMC
AQYwHQYDVR0OBBYEFFOSo33/gnbwM9TrkmdHYTMbaDsqMA0GCSqGSIb3DQEBBQUA
A4IBAQA5UFWd5EL/pBviIMm1zD2JLUCpp0mJG7JkwznIOzawhGmFFaxGoxAhQBEg
haP+E0KR66oAwVC6xe32QUVSHfWqWndzbODzLB8yj7WAR0cDM45ZngSBPBuFE3Wu
GLJX9g100ETfIX+4YBR/4NR/uvTnpnd9ete7Whl0ZfY94yuu4xQqB5QFv+P7IXXV
lTOjkjuGXEcyQAjQzbFaT9vIABSbeCXWBbjvOXukJy6WgAiclzGNSYprre8Ryydd
fmjW9HIGwsIO03EldivvqEYL1Hv1w/Pur+6FUEOaL68PEIUovfgwIB2BAw+vZDuw
cH0mX548PojGyg434cDjkSXa3mHF";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(_arg) => {
            let _result = load_system_root_store().unwrap();
        }
        _ => {
            let _result = load_embedded_cert().unwrap();
        }
    }
}

fn load_system_root_store() -> Result<rustls::RootCertStore, Box<dyn Error>> {
    let mut root_store = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs()? {
        root_store.add(&rustls::Certificate(cert.0))?;
    }
    Ok(root_store)
}

fn load_embedded_cert() -> Result<rustls::RootCertStore, Box<dyn Error>> {
    let mut root_store = rustls::RootCertStore::empty();
    let cert = b64.decode(EMBEDDED_CERT)?;
    root_store.add(&rustls::Certificate(cert))?;

    Ok(root_store)
}
