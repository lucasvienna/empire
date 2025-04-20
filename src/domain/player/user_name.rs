use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::AsExpression;
use unicode_segmentation::UnicodeSegmentation;

use crate::{ErrorKind, Result};

/// A type representing a validated username.
///
/// # Implementation Details
/// Internally wraps a `String` and implements validation rules for usernames.
/// Implements `diesel::AsExpression` for database compatibility and uses `Text` as the SQL type.
#[derive(AsExpression, Debug, Clone, PartialEq, Eq)]
#[diesel(sql_type = Text)]
pub struct UserName(String);

impl UserName {
    /// Attempts to create a new `Username` from a string, validating the input.
    ///
    /// # Arguments
    /// * `s` - A `String` containing the proposed username
    ///
    /// # Returns
    /// * `Ok(Username)` - If the string passes all validation rules
    /// * `Err(Error)` - If the string fails validation
    ///
    /// # Validation Rules
    /// - Must not be empty or only whitespace
    /// - Must not exceed 256 graphemes in length
    /// - Must not contain any of these characters: /, (, ), ", <, >, \, {, }
    pub fn parse(s: String) -> Result<UserName> {
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "player-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err((ErrorKind::InvalidUsername, "Invalid username").into())
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for UserName {
    /// Returns a reference to the underlying string.
    ///
    /// # Returns
    /// * `&str` - A reference to the username string
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToSql<Text, Pg> for UserName {
    /// Converts the username to its SQL representation.
    ///
    /// # Arguments
    /// * `out` - The output buffer for the SQL serialization
    ///
    /// # Returns
    /// * `diesel::serialize::Result` - The result of the SQL serialization
    fn to_sql<'a>(&'a self, out: &mut Output<'a, '_, Pg>) -> diesel::serialize::Result {
        <String as ToSql<Text, Pg>>::to_sql(&self.0, out)
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};

    use crate::domain::player::user_name::*;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(UserName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(UserName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(UserName::parse(name));
    }
    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(UserName::parse(name));
    }
    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(UserName::parse(name));
        }
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Bruce Wayne".to_string();
        assert_ok!(UserName::parse(name));
    }
}
