use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct XasidaLink {
    href: String,
    text: String,
}

impl XasidaLink {
    pub fn new(path: &str, text: &str) -> Self {
        let cleaned_path = path.strip_prefix("./").unwrap();
        let href = "http://khassidaenpdf.free.fr/".to_string() + cleaned_path;

        Self {
            href,
            text: text.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::xasida_link::XasidaLink;

    #[test]
    fn test_xasida_new() {
        assert_eq!(
            XasidaLink {
                href: "http://khassidaenpdf.free.fr/khassida_pdf/AL MUJIIBU.pdf".to_string(),
                text: "AL MUJIIBU.pdf".to_string()
            },
            XasidaLink::new("./khassida_pdf/AL MUJIIBU.pdf", "AL MUJIIBU.pdf")
        )
    }
}
