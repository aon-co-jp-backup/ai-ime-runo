use crate::backend::{ConversionBackend, ConversionContext, DictionaryOrderBackend};
use crate::dictionary::Dictionary;
use crate::input_mode::{suggest_input_mode, InputMode};

#[derive(Debug, Clone)]
pub struct ConversionResult {
    /// 文脈を踏まえて並べ替えられた変換候補(優先順)。
    pub candidates: Vec<String>,
    /// 最有力候補から提案される入力モード。
    pub suggested_mode: InputMode,
}

pub struct ConversionEngine<B: ConversionBackend = DictionaryOrderBackend> {
    dictionary: Dictionary,
    backend: B,
}

impl ConversionEngine<DictionaryOrderBackend> {
    /// 種辞書(シードデータ)+ 既定バックエンド(並べ替えなし)で構築する。
    pub fn with_seed_dictionary() -> Self {
        Self {
            dictionary: Dictionary::seed(),
            backend: DictionaryOrderBackend,
        }
    }
}

impl<B: ConversionBackend> ConversionEngine<B> {
    pub fn new(dictionary: Dictionary, backend: B) -> Self {
        Self { dictionary, backend }
    }

    /// `reading`(ひらがな読み)を、前後の文脈`context`を踏まえて変換する。
    /// 辞書に無い読みの場合は`None`(=通常のIMEの一般変換にフォールバックする想定)。
    pub fn convert(&self, reading: &str, context: &ConversionContext) -> Option<ConversionResult> {
        let entry = self.dictionary.lookup(reading)?;
        let ranked = self.backend.rerank(reading, context, entry.candidates.clone());
        let suggested_mode = ranked
            .first()
            .map(|c| suggest_input_mode(c))
            .unwrap_or(InputMode::Hiragana);
        Some(ConversionResult {
            candidates: ranked,
            suggested_mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sazan_aizu_converts_with_two_candidates_and_suggests_alphabet_mode() {
        let engine = ConversionEngine::with_seed_dictionary();
        let ctx = ConversionContext::default();
        let result = engine.convert("さざんあいず", &ctx).unwrap();
        assert_eq!(result.candidates, vec!["3X3 EYES", "サザンアイス"]);
        // 最有力候補("3X3 EYES")はアルファベットなので、Alphabetモードを提案する。
        assert_eq!(result.suggested_mode, InputMode::Alphabet);
    }

    #[test]
    fn sanja_yan_unkara_converts_and_stays_in_hiragana_mode() {
        let engine = ConversionEngine::with_seed_dictionary();
        let ctx = ConversionContext::default();
        let result = engine.convert("さんじゃやんうんから", &ctx).unwrap();
        assert_eq!(result.candidates, vec!["三只眼吽迦羅"]);
        assert_eq!(result.suggested_mode, InputMode::Hiragana);
    }

    #[test]
    fn hannya_shingyo_converts_correctly() {
        let engine = ConversionEngine::with_seed_dictionary();
        let ctx = ConversionContext::default();
        let result = engine.convert("はんにゃしんぎょう", &ctx).unwrap();
        assert_eq!(result.candidates, vec!["般若心経"]);
    }

    #[test]
    fn unknown_reading_returns_none_for_fallback_to_normal_ime() {
        let engine = ConversionEngine::with_seed_dictionary();
        let ctx = ConversionContext::default();
        assert!(engine.convert("みしらぬよみ", &ctx).is_none());
    }

    /// カスタムバックエンドへ差し替えられることを確認する(AIバックエンド
    /// 差し替えの拡張ポイントが実際に機能する証明)。
    #[test]
    fn custom_backend_can_rerank_candidates() {
        struct ReverseBackend;
        impl ConversionBackend for ReverseBackend {
            fn rerank(&self, _reading: &str, _context: &ConversionContext, mut candidates: Vec<String>) -> Vec<String> {
                candidates.reverse();
                candidates
            }
        }

        let engine = ConversionEngine::new(Dictionary::seed(), ReverseBackend);
        let ctx = ConversionContext::default();
        let result = engine.convert("さざんあいず", &ctx).unwrap();
        assert_eq!(result.candidates, vec!["サザンアイス", "3X3 EYES"]);
        // 最有力候補が入れ替わったので、提案モードもKatakanaに変わるはず。
        assert_eq!(result.suggested_mode, InputMode::Katakana);
    }
}
