//auth
fn auth() -> Result<Auth, Error> {
    let auth = Auth::new(
        &env::var("AWS_ACCESS_KEY_ID")?,
        &env::var("AWS_SECRET_ACCESS_KEY")?,
        None,
        None,
    );
    Ok(auth)
}