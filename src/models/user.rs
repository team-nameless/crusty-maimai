use crate::{content_process::Deserializer, MaimaiSession};

pub struct MaimaiUser {
    pub name: String,
    pub rating: i16

}

impl MaimaiUser {
    /// Populate user data.
    pub fn populate(session: MaimaiSession) -> Self {
        let html = session.jump_to(MaimaiSession::get_home_url());
        let deserializer = Deserializer::from_html(html);

        let name_divs = deserializer.search(".name_block")[0];
        let rating_divs = deserializer.search(".rating_block")[0];

        Self {
            name: String::from(name_divs.text().nth(0).unwrap()),
            rating: String::from(rating_divs.text().nth(0).unwrap()).parse().unwrap(),
        }
    }
}