use urlencoding::encode;
use std::error::Error;
use std::env;

pub fn login_redirect() -> Result<(), Box<dyn Error>> {
    let data_access_scope = encode(
        "https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/userinfo.email",
    );

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
         ?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
        env::var("CLIENT_ID")?,
        env::var("REDIRECT_URI")?,
        data_access_scope
    );

    println!("Membuka browser untuk autentikasi Google...");
    webbrowser::open(&auth_url)?;
    Ok(())
}