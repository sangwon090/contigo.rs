use base64;
use contigo::auth::*;

pub fn get_auth_client() -> AuthClient {
    let account = Account {
        email: "example@example.com".to_string(),
        password: "p@ssw0rd".to_string(),
    };

    let device = Device {
        uuid: base64::encode("00000000-0000-0000-0000-000000000000"),
        name: "contigo".to_string(),
        os_version: "10.0".to_string(),
    };

    AuthClient::new(account, device, true, true)
}

#[tokio::test]
async fn login() {
    let result = get_auth_client().login().await;
    println!("{:#?}", result.unwrap());
}

#[tokio::test]
async fn request_passcode() {
    let result = get_auth_client().request_passcode().await;
    println!("{:#?}", result.unwrap());
}

#[tokio::test]
async fn register_device() {
    let result = get_auth_client().register_device("1337".to_string()).await;
    println!("{:#?}", result.unwrap());
}
