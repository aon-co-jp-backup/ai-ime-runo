//! 入力モード(ひらがな/カタカナ/アルファベット)の自動判定。
//!
//! ユーザーがひらがなモードのままでも、変換候補の性質(外来語・商標名など)から
//! 「カタカナ/アルファベット表記の方が自然」と判断できる場合に提案する。
//! 判定ロジック自体は現時点では単純な文字種ヒューリスティックだが、将来的には
//! ここを契約不要の独自AI(またはローカルLLM/契約AI)に差し替える想定
//! ([`crate::backend::ConversionBackend`]と同様の考え方)。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Hiragana,
    Katakana,
    Alphabet,
}

fn is_katakana(c: char) -> bool {
    matches!(c, '\u{30A0}'..='\u{30FF}')
}

/// 変換候補の文字種から、最も自然な入力モードを提案する。
///
/// - 候補がASCII文字のみ(例: "3X3 EYES")なら`Alphabet`。
/// - 候補がカタカナ(と長音記号)のみなら`Katakana`。
/// - それ以外(漢字混じり等)は`Hiragana`のまま(通常のかな漢字変換の範囲)。
pub fn suggest_input_mode(candidate: &str) -> InputMode {
    let non_space: Vec<char> = candidate.chars().filter(|c| !c.is_whitespace()).collect();
    if non_space.is_empty() {
        return InputMode::Hiragana;
    }
    if non_space.iter().all(|c| c.is_ascii()) {
        InputMode::Alphabet
    } else if non_space.iter().all(|&c| is_katakana(c)) {
        InputMode::Katakana
    } else {
        InputMode::Hiragana
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_candidate_suggests_alphabet_mode() {
        assert_eq!(suggest_input_mode("3X3 EYES"), InputMode::Alphabet);
    }

    #[test]
    fn katakana_candidate_suggests_katakana_mode() {
        assert_eq!(suggest_input_mode("サザンアイス"), InputMode::Katakana);
    }

    #[test]
    fn kanji_candidate_stays_in_hiragana_mode() {
        assert_eq!(suggest_input_mode("三只眼吽迦羅"), InputMode::Hiragana);
        assert_eq!(suggest_input_mode("般若心経"), InputMode::Hiragana);
    }
}
