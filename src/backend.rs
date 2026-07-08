//! 変換候補の並べ替えを行うバックエンドの抽象化。
//!
//! 「無料AIから契約AIからオープンソースのローカルLLMまで簡単連携」という
//! 要求に対応するため、辞書検索の後段(文脈を踏まえた候補の並べ替え)を
//! トレイトとして切り出す。デフォルトは辞書の登録順をそのまま返す
//! [`DictionaryOrderBackend`]だが、将来的に以下のような実装へ差し替えられる:
//!
//! - 無料AI API(HTTPクライアントで問い合わせ)
//! - 契約(有料)AI API
//! - オープンソースのローカルLLM(open-raid-zのCopilotアドバイザーで実装した
//!   ローカルLLM検知([Ollama/LM Studio等])と同じ考え方を再利用できる)
//!
//! 呼び出し側([`crate::ConversionEngine`])はトレイトにしか依存しないため、
//! バックエンドの差し替えに伴う変更は不要。

/// 変換候補を並べ替える際に参照する前後の文脈。
#[derive(Debug, Clone, Copy, Default)]
pub struct ConversionContext<'a> {
    pub preceding_text: &'a str,
    pub following_text: &'a str,
}

pub trait ConversionBackend {
    /// 辞書から得られた候補(登録順)を、文脈を踏まえて並べ替える。
    /// 既定実装は何もせず、そのまま返す(=辞書順を尊重)。
    fn rerank(&self, reading: &str, context: &ConversionContext, candidates: Vec<String>) -> Vec<String> {
        let _ = (reading, context);
        candidates
    }
}

/// 既定のバックエンド: 辞書の登録順をそのまま返すだけの、何もしない実装。
/// AIバックエンドが未設定の場合のフォールバックとして使う。
pub struct DictionaryOrderBackend;

impl ConversionBackend for DictionaryOrderBackend {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_order_backend_returns_candidates_unchanged() {
        let backend = DictionaryOrderBackend;
        let ctx = ConversionContext::default();
        let candidates = vec!["3X3 EYES".to_string(), "サザンアイス".to_string()];
        assert_eq!(backend.rerank("さざんあいず", &ctx, candidates.clone()), candidates);
    }
}
