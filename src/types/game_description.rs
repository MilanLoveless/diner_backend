use unicode_segmentation::UnicodeSegmentation;

pub struct GameDescription(String);

impl GameDescription {
    /// Returns an instance of `SubscriberName` if the input satisfies all
    /// our validation constraints on subscriber names.
    /// It panics otherwise.
    pub fn parse(s: String) -> Result<GameDescription, String> {
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

impl AsRef<str> for GameDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
