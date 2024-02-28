#[cfg(test)]
mod test_deserialize {
    use dotenv::dotenv;
    use crusty_maimai::{content_process::Deserializer, *};

    #[test]
    fn logged_in_serialize_works() {
        dotenv().ok();

        let session = MaimaiSession::default();
        let username = std::env::var("SEGA_ID_USERNAME").unwrap();
        let password = std::env::var("SEGA_ID_PASSWORD").unwrap();

        let html = session.login_with_credentials(username, password);

        let deserializer = Deserializer::from_html(html);

        let divs = deserializer.search(".name_block")[0];

        if let Some(text) = divs.text().nth(0) {
            assert_ne!(text, "");
        }
    }
}
