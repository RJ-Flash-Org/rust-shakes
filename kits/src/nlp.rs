//! NLP kit (MVP: tokenization utilities).

/// Simple whitespace tokenizer.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace().map(|s| s.to_lowercase()).collect()
}
