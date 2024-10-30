use std::cmp::Ordering;

use super::{pure_tone::ToneAndDuration, text_analyzer::TextAnalyzer};

#[derive(Clone, Debug, PartialEq)]
enum TrackType {
    Hiragana,
    Katakana,
    Kanji,
    Alphabets,
}

pub struct Text2Track {
    pub text_analyzer: TextAnalyzer,
}

impl Text2Track {
    pub fn new(text_analyzer: TextAnalyzer) -> Self {
        Self { text_analyzer }
    }

    pub fn generate_track(&self) -> Vec<ToneAndDuration> {
        let text_length = self.text_analyzer.length();
        let hiragana_ratio = self.text_analyzer.calculate_hiragana_ratio();
        let katakana_ratio = self.text_analyzer.calculate_katakana_ratio();
        let kanji_ratio = self.text_analyzer.calculate_kanji_ratio();
        let alphabets_ratio = self.text_analyzer.calculate_alphabets_ratio();

        let track = match self.determine_track_type() {
            TrackType::Hiragana => self.generate_track_hiragana(),
            TrackType::Katakana => self.generate_track_katakana(),
            TrackType::Kanji => self.generate_track_kanji(),
            TrackType::Alphabets => self.generate_track_alphabets(),
        };

        track
    }

    fn determine_track_type(&self) -> TrackType {
        let hiragana_ratio = self.text_analyzer.calculate_hiragana_ratio();
        let katakana_ratio = self.text_analyzer.calculate_katakana_ratio();
        let kanji_ratio = self.text_analyzer.calculate_kanji_ratio();
        let alphabets_ratio = self.text_analyzer.calculate_alphabets_ratio();

        let binding = [
            (hiragana_ratio, TrackType::Hiragana),
            (katakana_ratio, TrackType::Katakana),
            (kanji_ratio, TrackType::Kanji),
            (alphabets_ratio, TrackType::Alphabets),
        ];
        let track_type = binding
            .iter()
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
            .map(|(_, track_type)| track_type)
            .unwrap_or(&TrackType::Alphabets);

        track_type.clone()
    }

    fn generate_track_hiragana(&self) -> Vec<ToneAndDuration> {
        vec![
            // 1小節目 - 導入
            ToneAndDuration {
                frequency: 261.63,
                duration: 1.0 / 2.0,
            }, // ド
            ToneAndDuration {
                frequency: 293.66,
                duration: 1.0 / 2.0,
            }, // レ
            // 2小節目
            ToneAndDuration {
                frequency: 349.23,
                duration: 3.0 / 4.0,
            }, // ファ
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 4.0,
            }, // ソ
            // 3小節目
            ToneAndDuration {
                frequency: 349.23,
                duration: 1.0 / 2.0,
            }, // ファ
            ToneAndDuration {
                frequency: 293.66,
                duration: 1.0 / 2.0,
            }, // レ
            // 4小節目
            ToneAndDuration {
                frequency: 261.63,
                duration: 1.0,
            }, // ド
            // 5小節目
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 2.0,
            }, // ソ
            ToneAndDuration {
                frequency: 349.23,
                duration: 1.0 / 2.0,
            }, // ファ
            // 6小節目
            ToneAndDuration {
                frequency: 293.66,
                duration: 3.0 / 4.0,
            }, // レ
            ToneAndDuration {
                frequency: 261.63,
                duration: 1.0 / 4.0,
            }, // ド
            // 7小節目
            ToneAndDuration {
                frequency: 293.66,
                duration: 1.0 / 2.0,
            }, // レ
            ToneAndDuration {
                frequency: 349.23,
                duration: 1.0 / 2.0,
            }, // ファ
            // 8小節目
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0,
            }, // ソ
            // 9小節目 - 第二楽章
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // 高いド
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 2.0,
            }, // ソ
            // 10小節目
            ToneAndDuration {
                frequency: 349.23,
                duration: 3.0 / 4.0,
            }, // ファ
            ToneAndDuration {
                frequency: 293.66,
                duration: 1.0 / 4.0,
            }, // レ
            // 11小節目
            ToneAndDuration {
                frequency: 261.63,
                duration: 1.0,
            }, // ド
            // 12小節目
            ToneAndDuration {
                frequency: 293.66,
                duration: 1.0,
            }, // レ
            // 13小節目
            ToneAndDuration {
                frequency: 349.23,
                duration: 1.0 / 2.0,
            }, // ファ
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 2.0,
            }, // ソ
            // 14小節目
            ToneAndDuration {
                frequency: 349.23,
                duration: 3.0 / 4.0,
            }, // ファ
            ToneAndDuration {
                frequency: 293.66,
                duration: 1.0 / 4.0,
            }, // レ
            // 15小節目
            ToneAndDuration {
                frequency: 261.63,
                duration: 1.0 / 2.0,
            }, // ド
            ToneAndDuration {
                frequency: 293.66,
                duration: 1.0 / 2.0,
            }, // レ
            // 16小節目 - 終結
            ToneAndDuration {
                frequency: 261.63,
                duration: 1.0,
            }, // ド
        ]
    }

    fn generate_track_katakana(&self) -> Vec<ToneAndDuration> {
        vec![
            // 1小節目 - 導入句
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 3.0,
            }, // G
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 6.0,
            }, // A
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 2.0,
            }, // B
            // 2小節目
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 3.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 2.0,
            }, // A
            // 3小節目
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 6.0,
            }, // G
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 6.0,
            }, // A
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // C
            // 4小節目
            ToneAndDuration {
                frequency: 493.88,
                duration: 3.0 / 4.0,
            }, // B
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 4.0,
            }, // A
            // 5小節目 - メインテーマ
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 6.0,
            }, // C
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 6.0,
            }, // D
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 6.0,
            }, // E
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 2.0,
            }, // D
            // 6小節目
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 3.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 2.0,
            }, // A
            // 7小節目
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 6.0,
            }, // G
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 6.0,
            }, // A
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // C
            // 8小節目
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0,
            }, // B
            // 9小節目 - 展開部
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 3.0,
            }, // E
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 6.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // C
            // 10小節目
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 6.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 2.0,
            }, // A
            // 11小節目
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 6.0,
            }, // C
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 6.0,
            }, // D
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 6.0,
            }, // E
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 2.0,
            }, // D
            // 12小節目
            ToneAndDuration {
                frequency: 523.25,
                duration: 3.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            // 13小節目 - クライマックス
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 6.0,
            }, // E
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 6.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 6.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 2.0,
            }, // B
            // 14小節目
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 3.0,
            }, // A
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // C
            // 15小節目 - エンディング
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 6.0,
            }, // B
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 6.0,
            }, // A
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 6.0,
            }, // G
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 2.0,
            }, // A
            // 16小節目
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0,
            }, // G
        ]
    }

    fn generate_track_kanji(&self) -> Vec<ToneAndDuration> {
        vec![
            // 1小節目 - 壮大な導入
            ToneAndDuration {
                frequency: 392.00,
                duration: 3.0 / 4.0,
            }, // G (宮)
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 4.0,
            }, // A (商)
            // 2小節目
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // C (角)
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 2.0,
            }, // D (徴)
            // 3小節目 - 優雅な旋律
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 4.0,
            }, // A
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 2.0,
            }, // G
            // 4小節目
            ToneAndDuration {
                frequency: 329.63,
                duration: 3.0 / 4.0,
            }, // E (羽)
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 4.0,
            }, // G
            // 5小節目 - メインテーマ
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // C
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            // 6小節目
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0,
            }, // A
            // 7小節目 - 装飾的フレーズ
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 4.0,
            }, // E
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            // 8小節目
            ToneAndDuration {
                frequency: 440.00,
                duration: 3.0 / 4.0,
            }, // A
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 4.0,
            }, // G
            // 9小節目 - 山水画のような表現
            ToneAndDuration {
                frequency: 783.99,
                duration: 1.0 / 2.0,
            }, // G (高音)
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 2.0,
            }, // E
            // 10小節目
            ToneAndDuration {
                frequency: 587.33,
                duration: 3.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            // 11小節目 - 力強い展開
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 2.0,
            }, // E
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            // 12小節目
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0,
            }, // A
            // 13小節目 - 水墨画のような繊細さ
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 2.0,
            }, // E
            // 14小節目
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 2.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 2.0,
            }, // C
            // 15小節目 - エンディングへ
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 2.0,
            }, // A
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 2.0,
            }, // G
            // 16小節目 - 荘厳な終結
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0,
            }, // G
        ]
    }

    fn generate_track_alphabets(&self) -> Vec<ToneAndDuration> {
        vec![
            // 1小節目 - スイングのある導入
            ToneAndDuration {
                frequency: 493.88,
                duration: 3.0 / 8.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 8.0,
            }, // C
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            // 2小節目 - ブルーノートを使用
            ToneAndDuration {
                frequency: 466.16,
                duration: 1.0 / 4.0,
            }, // Bb (ブルーノート)
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 2.0,
            }, // A
            // 3小節目 - クロマティックアプローチ
            ToneAndDuration {
                frequency: 392.00,
                duration: 1.0 / 4.0,
            }, // G
            ToneAndDuration {
                frequency: 415.30,
                duration: 1.0 / 8.0,
            }, // Ab
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 8.0,
            }, // A
            ToneAndDuration {
                frequency: 466.16,
                duration: 1.0 / 4.0,
            }, // Bb
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            // 4小節目 - ターンアラウンド
            ToneAndDuration {
                frequency: 523.25,
                duration: 3.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            // 5小節目 - II-V-I進行を意識
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 4.0,
            }, // E
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            // 6小節目 - シンコペーション
            ToneAndDuration {
                frequency: 493.88,
                duration: 3.0 / 8.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 3.0 / 8.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            // 7小節目 - モーダルな展開
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 4.0,
            }, // A
            ToneAndDuration {
                frequency: 466.16,
                duration: 1.0 / 4.0,
            }, // Bb
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            // 8小節目 - ブリッジへの移行
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0,
            }, // D
            // 9小節目 - ブリッジ部分
            ToneAndDuration {
                frequency: 698.46,
                duration: 1.0 / 4.0,
            }, // F
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 4.0,
            }, // E
            ToneAndDuration {
                frequency: 622.25,
                duration: 1.0 / 4.0,
            }, // Eb
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            // 10小節目 - アルペジオ風
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 4.0,
            }, // D
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 4.0,
            }, // E
            ToneAndDuration {
                frequency: 698.46,
                duration: 1.0 / 4.0,
            }, // F
            // 11小節目 - スケールの下降
            ToneAndDuration {
                frequency: 783.99,
                duration: 3.0 / 8.0,
            }, // G
            ToneAndDuration {
                frequency: 739.99,
                duration: 1.0 / 8.0,
            }, // F#
            ToneAndDuration {
                frequency: 698.46,
                duration: 1.0 / 4.0,
            }, // F
            ToneAndDuration {
                frequency: 659.26,
                duration: 1.0 / 4.0,
            }, // E
            // 12小節目 - モダンなフレーズ
            ToneAndDuration {
                frequency: 622.25,
                duration: 1.0 / 2.0,
            }, // Eb
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 2.0,
            }, // D
            // 13小節目 - 終結部への導入
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            ToneAndDuration {
                frequency: 466.16,
                duration: 1.0 / 4.0,
            }, // Bb
            ToneAndDuration {
                frequency: 440.00,
                duration: 1.0 / 4.0,
            }, // A
            // 14小節目 - 洗練されたエンディング
            ToneAndDuration {
                frequency: 392.00,
                duration: 3.0 / 8.0,
            }, // G
            ToneAndDuration {
                frequency: 440.00,
                duration: 3.0 / 8.0,
            }, // A
            ToneAndDuration {
                frequency: 466.16,
                duration: 1.0 / 4.0,
            }, // Bb
            // 15小節目
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
            ToneAndDuration {
                frequency: 523.25,
                duration: 1.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 587.33,
                duration: 1.0 / 2.0,
            }, // D
            // 16小節目 - ジャズらしい終止
            ToneAndDuration {
                frequency: 523.25,
                duration: 3.0 / 4.0,
            }, // C
            ToneAndDuration {
                frequency: 493.88,
                duration: 1.0 / 4.0,
            }, // B
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let text_analyzer = TextAnalyzer::new("こんにちは".to_string());
        let text2track = Text2Track::new(text_analyzer);
        assert_eq!(text2track.text_analyzer.text, "こんにちは".to_string());
    }

    #[rstest]
    #[case::hiragana("こんにちは、私の名前はおもちです。", vec![ToneAndDuration{ frequency: 261.63, duration: 1.0 / 4.0 * 1.5 }])]
    #[case::katakana("ヘイ！元気デスカ？", vec![ToneAndDuration{ frequency: 261.63, duration: 1.0 / 4.0 * 1.5 }])]
    #[case::kanji("東京特許許可局に行く", vec![ToneAndDuration{ frequency: 261.63, duration: 1.0 / 4.0 * 1.5 }])]
    #[case::alphabets("Why Japanese people!?", vec![ToneAndDuration{ frequency: 261.63, duration: 1.0 / 4.0 * 1.5 }])]
    fn test_generate_track(#[case] input: String, #[case] expected: Vec<ToneAndDuration>) {
        let text_analyzer = TextAnalyzer::new(input);
        let text2track = Text2Track::new(text_analyzer);
        let track = text2track.generate_track();

        assert_eq!(track, expected);
    }

    #[rstest]
    #[case::hiragana("こんにちは、私の名前はおもちです。", TrackType::Hiragana)]
    #[case::katakana("ヘイ！元気デスカ？", TrackType::Katakana)]
    #[case::kanji("東京特許許可局に行く", TrackType::Kanji)]
    #[case::alphabets("Why Japanese people!?", TrackType::Alphabets)]
    fn test_determine_track_type(#[case] input: String, #[case] expected: TrackType) {
        let text_analyzer = TextAnalyzer::new(input);
        let text2track = Text2Track::new(text_analyzer);
        let track_type = text2track.determine_track_type();

        assert_eq!(track_type, expected);
    }
}
