//! ひな型段階の辞書。
//!
//! 【将来計画】本番では、有名な小説・漫画・アニメ・GAME・韓国ドラマなどの
//! 地名・人名・キャラクター名・必殺技名や、漢方薬・化学薬品・サプリ・化粧品名を、
//! クラウド上のWEBサイト(自動クローラー)が収集して供給する想定
//! ([`project_ai_ime_runo`]メモリ参照)。本ファイルはそれまでの間、
//! 手動で登録した種辞書(シードデータ)を保持する。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryCategory {
    PlaceName,
    PersonName,
    Character,
    Skill,
    Medicine,
    Chemical,
    Supplement,
    Cosmetic,
    Sutra,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub reading: String,
    /// 変換候補(優先順)。1つの読みに複数の意味がある場合は複数登録する
    /// (例:「さざんあいず」→「3X3 EYES」と「サザンアイス」の両方)。
    pub candidates: Vec<String>,
    pub category: EntryCategory,
    pub source: Option<String>,
}

pub struct Dictionary {
    entries: HashMap<String, DictionaryEntry>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, entry: DictionaryEntry) {
        self.entries.insert(entry.reading.clone(), entry);
    }

    pub fn lookup(&self, reading: &str) -> Option<&DictionaryEntry> {
        self.entries.get(reading)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 手動登録した種辞書(シードデータ)。要求されていた具体例を含む。
    pub fn seed() -> Self {
        let mut dict = Self::new();

        dict.register(DictionaryEntry {
            reading: "さざんあいず".to_string(),
            candidates: vec!["3X3 EYES".to_string(), "サザンアイス".to_string()],
            category: EntryCategory::Character,
            source: Some("有名漫画/アニメ".to_string()),
        });

        // ユーザー指定の表記ゆれ2種("さんじゃ.../さんじや...")を両方登録する。
        for reading in ["さんじゃやんうんから", "さんじやんうんから"] {
            dict.register(DictionaryEntry {
                reading: reading.to_string(),
                candidates: vec!["三只眼吽迦羅".to_string()],
                category: EntryCategory::Character,
                source: None,
            });
        }

        dict.register(DictionaryEntry {
            reading: "はんにゃしんぎょう".to_string(),
            candidates: vec!["般若心経".to_string()],
            category: EntryCategory::Sutra,
            source: Some("仏教経典".to_string()),
        });

        dict
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sazan_aizu_has_two_candidates_in_priority_order() {
        let dict = Dictionary::seed();
        let entry = dict.lookup("さざんあいず").unwrap();
        assert_eq!(entry.candidates, vec!["3X3 EYES", "サザンアイス"]);
    }

    #[test]
    fn sanja_yan_unkara_variants_both_resolve_to_the_same_kanji() {
        let dict = Dictionary::seed();
        assert_eq!(
            dict.lookup("さんじゃやんうんから").unwrap().candidates,
            vec!["三只眼吽迦羅"]
        );
        assert_eq!(
            dict.lookup("さんじやんうんから").unwrap().candidates,
            vec!["三只眼吽迦羅"]
        );
    }

    #[test]
    fn hannya_shingyo_is_registered() {
        let dict = Dictionary::seed();
        assert_eq!(dict.lookup("はんにゃしんぎょう").unwrap().candidates, vec!["般若心経"]);
    }

    #[test]
    fn unknown_reading_returns_none() {
        let dict = Dictionary::seed();
        assert!(dict.lookup("そんざいしないよみ").is_none());
    }
}
