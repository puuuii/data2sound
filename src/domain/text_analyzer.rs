pub struct TextAnalyzer {
    pub text: String,
}

impl TextAnalyzer {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn length(&self) -> usize {
        self.text.chars().count()
    }

    pub fn calculate_hiragana_ratio(&self) -> f32 {
        self.count_hiragana() as f32 / self.length() as f32
    }

    pub fn calculate_katakana_ratio(&self) -> f32 {
        self.count_katakana() as f32 / self.length() as f32
    }

    pub fn calculate_kanji_ratio(&self) -> f32 {
        self.count_kanji() as f32 / self.length() as f32
    }

    pub fn calculate_alphabets_ratio(&self) -> f32 {
        self.count_alphabets() as f32 / self.length() as f32
    }

    fn count_hiragana(&self) -> usize {
        self.text
            .chars()
            .filter(|c| ('\u{3040}'..='\u{309F}').contains(c))
            .count()
    }

    fn count_katakana(&self) -> usize {
        self.text
            .chars()
            .filter(|c| ('\u{30A0}'..='\u{30FF}').contains(c))
            .count()
    }

    fn count_kanji(&self) -> usize {
        self.text
            .chars()
            .filter(|c| ('\u{4E00}'..='\u{9FFF}').contains(c))
            .count()
    }

    fn count_alphabets(&self) -> usize {
        self.text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text2param() {
        let text2param = TextAnalyzer::new("Hello, world!".to_string());
        assert_eq!(text2param.text, "Hello, world!".to_string());
    }

    #[test]
    fn test_length() {
        let cases = vec![
            ("こんにちは, world!", 13),
            ("私の名前はジョンです", 10),
            ("東京五輪は2021年に延期された", 16),
            ("Jupyter NotebookはPythonで使えます", 28),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.length(), expected);
        }
    }

    #[test]
    fn test_count_hiragana() {
        let cases = vec![
            ("こんにちは, world!", 5),
            ("私の名前はジョンです", 4),
            ("東京五輪は2021年に延期された", 5),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.count_hiragana(), expected);
        }
    }

    #[test]
    fn test_count_katakana() {
        let cases = vec![
            ("こんにちは, world!", 0),
            ("私の名前はジョンです", 3),
            ("東京五輪は2021年に延期された", 0),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.count_katakana(), expected);
        }
    }

    #[test]
    fn test_count_kanji() {
        let cases = vec![
            ("こんにちは, world!", 0),
            ("私の名前はジョンです", 3),
            ("東京五輪は2021年に延期された", 7),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.count_kanji(), expected);
        }
    }

    #[test]
    fn test_count_alphabets() {
        let cases = vec![
            ("こんにちは, world!", 5),
            ("私の名前はジョンです", 0),
            ("東京五輪は2021年に延期された", 0),
            ("Jupyter NotebookはPythonで使えます", 21),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.count_alphabets(), expected);
        }
    }

    #[test]
    fn test_calculate_hiragana_ratio() {
        let cases = vec![
            ("こんにちは, world!", 5.0 / 13.0),
            ("私の名前はジョンです", 4.0 / 10.0),
            ("東京五輪は2021年に延期された", 5.0 / 16.0),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.calculate_hiragana_ratio(), expected);
        }
    }

    #[test]
    fn test_calculate_katakana_ratio() {
        let cases = vec![
            ("こんにちは, world!", 0.0 / 13.0),
            ("私の名前はジョンです", 3.0 / 10.0),
            ("東京五輪は2021年に延期された", 0.0 / 16.0),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.calculate_katakana_ratio(), expected);
        }
    }

    #[test]
    fn test_calculate_kanji_ratio() {
        let cases = vec![
            ("こんにちは, world!", 0.0 / 13.0),
            ("私の名前はジョンです", 3.0 / 10.0),
            ("東京五輪は2021年に延期された", 7.0 / 16.0),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.calculate_kanji_ratio(), expected);
        }
    }

    #[test]
    fn test_calculate_alphabets_ratio() {
        let cases = vec![
            ("こんにちは, world!", 5.0 / 13.0),
            ("私の名前はジョンです", 0.0 / 10.0),
            ("東京五輪は2021年に延期された", 0.0 / 16.0),
            ("Jupyter NotebookはPythonで使えます", 21.0 / 28.0),
        ];

        for (text, expected) in cases {
            let text2param = TextAnalyzer::new(text.to_string());
            assert_eq!(text2param.calculate_alphabets_ratio(), expected);
        }
    }
}
