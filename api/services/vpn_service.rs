use x25519_dalek::{StaticSecret, PublicKey};
use rand::rngs::OsRng;
use base64::{encode, decode};

pub fn generate_keypair() -> (String, String) {
    let private = StaticSecret::new(OsRng);
    let public = PublicKey::from(&private);
    (encode(private.to_bytes()), encode(public.to_bytes()))
}

pub fn generate_wireguard_config(private_key: &str, server_ip: &str, server_public_key: &str, client_ip: &str) -> String {
    format!(
        "[Interface]\nPrivateKey = {}\nAddress = {}\n\n[Peer]\nPublicKey = {}\nEndpoint = {}:51820\nAllowedIPs = 0.0.0.0/0\n",
        private_key, client_ip, server_public_key, server_ip
    )
}