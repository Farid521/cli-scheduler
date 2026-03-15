use urlencoding::encode;

pub fn login_redirect() {
    let client_id = "732745752118-rk73e8rsdqln5t5jhp60a89vmmv4n8tp.apps.googleusercontent.com";
    let redirect_uri = "http://localhost:8090";

    let data_acces_scope = encode("https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/userinfo.email");

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline",
        client_id,
        redirect_uri,
        data_acces_scope
    );



    print!("opening browser");
    open::that(auth_url).unwrap();
}