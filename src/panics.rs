use core::iter::Peekable;

use proc_macro::{Span, TokenStream, TokenTree};

use crate::doc_section::{DocSection, HeadingLevel, ListStyle};
use crate::helper::{
    expect_end, generate_compile_error, generate_doc_attribute, parse_string_literal_list,
};

pub fn generate_panics_docs(
    cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream, TokenStream> {
    if let Some(sentinel) = PanicSentinel::from_token(cursor.peek()) {
        cursor.next();
        expect_end(cursor, sentinel.exclusive_error())?;
        return Ok(generate_doc_attribute(sentinel.documentation()));
    }

    let literals = parse_string_literal_list(cursor)?;
    expect_end(cursor, "unexpected token after panic conditions")?;

    if literals.is_empty() {
        return Err(generate_compile_error(
            Span::call_site(),
            "#[panics(...)] requires at least one condition string, or one of the bare sentinels: 'never', 'none', or 'always'",
        ));
    }

    let items = literals
        .into_iter()
        .map(|lit| lit.to_string().trim_matches('"').to_string())
        .collect();

    Ok(DocSection {
        title: "Panics",
        heading_level: HeadingLevel::H1,
        preamble: Some("This function panics when:"),
        style: ListStyle::Numbered,
        items,
    }
    .render())
}

#[derive(Clone, Copy)]
enum PanicSentinel {
    Never,
    Always,
}

impl PanicSentinel {
    fn from_token(token: Option<&TokenTree>) -> Option<Self> {
        let Some(TokenTree::Ident(ident)) = token else {
            return None;
        };

        match ident.to_string().as_str() {
            "never" | "none" => Some(Self::Never),
            "always" => Some(Self::Always),
            _ => None,
        }
    }

    fn exclusive_error(self) -> &'static str {
        match self {
            Self::Never => "the 'never'/'none' sentinel is exclusive; remove additional arguments",
            Self::Always => "the 'always' sentinel is exclusive; remove additional arguments",
        }
    }

    fn documentation(self) -> &'static str {
        match self {
            Self::Never => "# Panics\n\nThis function does not panic.",
            Self::Always => "# Panics\n\nThis function always panics.",
        }
    }
}
