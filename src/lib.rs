//! ai-ime-runo: 前後の文脈からAIが変換候補を用意するIME(ひな型段階)。
//!
//! 【現状のスコープ】種辞書(シードデータ)+ 文字種ベースの入力モード提案
//! + 差し替え可能な変換バックエンド、という土台のみ。
//!
//! 【未実装(今後の拡張)】
//! - オンラインクラウドでの自動クロール辞書収集WEBサイト
//! - 実際の前後文脈解析(現状は`ConversionContext`を受け取るが、既定の
//!   [`backend::DictionaryOrderBackend`]は中身を見ない)
//! - 無料AI/契約AI/ローカルLLMとの実際の接続(トレイトの土台のみ用意)

pub mod backend;
pub mod dictionary;
pub mod engine;
pub mod input_mode;

pub use backend::{ConversionBackend, ConversionContext, DictionaryOrderBackend};
pub use dictionary::{Dictionary, DictionaryEntry, EntryCategory};
pub use engine::{ConversionEngine, ConversionResult};
pub use input_mode::{suggest_input_mode, InputMode};
