use urlencoding::encode;
use webbrowser;
use std::io;
use std::error::Error;
use std::env;

pub fn login_redirect() -> Result<(), Box<dyn Error>> {
    let data_acces_scope = encode("https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/userinfo.email");

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline",
        env::var("CLIENT_ID")?,
        env::var("REDIRECT_URI")?,
        data_acces_scope
    );
    println!("opening browser");

    webbrowser::open(&auth_url)?;
    Ok(())
}