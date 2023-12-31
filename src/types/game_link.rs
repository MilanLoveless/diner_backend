use unicode_segmentation::UnicodeSegmentation;
use url::Url;

pub struct GameLink(String);

impl GameLink {
    /// Returns an instance of `GameLink` if the input satisfies all
    /// our validation constraints on links.
    /// It returns an Err otherwise.
    pub fn parse(s: String) -> Result<GameLink, String> {
        if Url::parse(s.as_str()).is_err() {
            return Err("Invalid url".to_string());
        };
        // `.trim()` returns a view over the input `s` without trailing
        // whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();
        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set, // the recommended one.
        let is_too_long = s.graphemes(true).count() > 2048;

        if is_empty_or_whitespace || is_too_long {
            Err(format!("{} is not a valid game name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for GameLink {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
