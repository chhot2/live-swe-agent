//! Text Analyzer Module
//!
//! Provides text analysis and statistics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct TextAnalyzer;

/// Text statistics result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a TextStatistics.
pub struct TextStatistics {
    pub char_count: usize,
    pub word_count: usize,
    pub sentence_count: usize,
    pub paragraph_count: usize,
    pub avg_word_length: f64,
    pub avg_sentence_length: f64,
    pub reading_time_minutes: u32,
    pub speaking_time_minutes: u32,
    pub unique_words: usize,
    pub lexical_density: f64,
    pub readability_score: f64,
}

impl TextAnalyzer {
    /// Analyze text and return statistics
    pub fn analyze(text: &str) -> TextStatistics {
        let chars: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();
        let words: Vec<&str> = text.split_whitespace().collect();
        let sentences: Vec<&str> = text
            .split(['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .collect();
        let paragraphs: Vec<&str> = text
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        let word_count = words.len();
        let sentence_count = sentences.len().max(1);

        let avg_word_length = if word_count > 0 {
            words.iter().map(|w| w.len() as f64).sum::<f64>() / word_count as f64
        } else {
            0.0
        };

        let avg_sentence_length = word_count as f64 / sentence_count as f64;

        // Reading time: ~200 words per minute
        let reading_time = (word_count as f64 / 200.0).ceil() as u32;

        // Speaking time: ~150 words per minute
        let speaking_time = (word_count as f64 / 150.0).ceil() as u32;

        // Unique words and lexical density
        let lowercase_words: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
        let unique: std::collections::HashSet<&str> =
            lowercase_words.iter().map(|s| s.as_str()).collect();
        let unique_words = unique.len();
        let lexical_density = if word_count > 0 {
            unique_words as f64 / word_count as f64
        } else {
            0.0
        };

        // Flesch Reading Ease approximation
        let syllables_per_word = avg_word_length / 3.0; // Rough approximation
        let readability_score = 206.835 - 1.015 * avg_sentence_length - 84.6 * syllables_per_word;
        let readability_score = readability_score.clamp(0.0, 100.0);

        TextStatistics {
            char_count: chars.len(),
            word_count,
            sentence_count,
            paragraph_count: paragraphs.len(),
            avg_word_length,
            avg_sentence_length,
            reading_time_minutes: reading_time,
            speaking_time_minutes: speaking_time,
            unique_words,
            lexical_density,
            readability_score,
        }
    }

    /// Get readability level description
    /// Get the readability level for a given score
    ///
    /// # Arguments
    /// * `score` - The readability score
    pub fn readability_level(score: f64) -> &'static str {
        match score as i32 {
            90..=100 => "Very Easy (5th grade)",
            80..=89 => "Easy (6th grade)",
            70..=79 => "Fairly Easy (7th grade)",
            60..=69 => "Standard (8th-9th grade)",
            50..=59 => "Fairly Difficult (10th-12th grade)",
            30..=49 => "Difficult (College)",
            _ => "Very Difficult (Professional)",
        }
    }

    /// Extract keywords from text (simple TF-based extraction)
    /// Extract keywords from text
    ///
    /// # Arguments
    /// * `text` - The text to extract keywords from
    /// * `max_keywords` - Maximum number of keywords to return
    pub fn extract_keywords(text: &str, max_keywords: usize) -> Vec<(String, usize)> {
        let stop_words: std::collections::HashSet<&str> = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
            "by", "from", "as", "is", "was", "are", "were", "been", "be", "have", "has", "had",
            "do", "does", "did", "will", "would", "could", "should", "may", "might", "must",
            "shall", "can", "this", "that", "these", "those", "it", "its", "they", "them", "their",
            "we", "us", "our", "you", "your", "he", "she", "him", "her", "his", "i", "me", "my",
            "what", "which", "who", "whom", "whose", "where", "when", "why", "how", "all", "each",
            "every", "both", "few", "more", "most", "other", "some", "such", "no", "not", "only",
            "own", "same", "so", "than", "too", "very", "just",
        ]
        .iter()
        .cloned()
        .collect();

        let words: Vec<String> = text
            .to_lowercase()
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| w.len() > 2 && !stop_words.contains(w))
            .map(|s| s.to_string())
            .collect();

        let mut freq: HashMap<String, usize> = HashMap::new();
        for word in words {
            *freq.entry(word).or_insert(0) += 1;
        }

        let mut sorted: Vec<(String, usize)> = freq.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(max_keywords);
        sorted
    }
}

#[cfg(test)]
mod enhanced_content_tests {
    use super::super::{AdvancedLanguageDetector, ScriptType};
    use super::*;

    #[test]
    fn test_advanced_language_detection() {
        let detector = AdvancedLanguageDetector::new();

        let english_text = "The quick brown fox jumps over the lazy dog. This is a test sentence with common English words.";
        let result = detector.detect(english_text);
        assert_eq!(result.language, "en");
        assert!(result.confidence > 0.3);

        let spanish_text = "El rápido zorro marrón salta sobre el perro perezoso. Esta es una prueba de texto en español.";
        let result = detector.detect(spanish_text);
        assert_eq!(result.language, "es");
    }

    #[test]
    fn test_script_detection() {
        let detector = AdvancedLanguageDetector::new();

        assert_eq!(detector.detect_script("Hello World"), ScriptType::Latin);
        assert_eq!(detector.detect_script("Привет мир"), ScriptType::Cyrillic);
        assert_eq!(detector.detect_script("你好世界"), ScriptType::CJK);
    }

    #[test]
    fn test_text_statistics() {
        let text = "This is a test. It has multiple sentences. The analysis should work correctly.";
        let stats = TextAnalyzer::analyze(text);

        assert_eq!(stats.sentence_count, 3);
        assert!(stats.word_count > 0);
        assert!(stats.readability_score > 0.0);
    }

    #[test]
    fn test_keyword_extraction() {
        let text = "Programming programming is fun. Rust programming language is great for systems programming.";
        let keywords = TextAnalyzer::extract_keywords(text, 5);

        assert!(!keywords.is_empty());
        assert!(keywords[0].0 == "programming");
    }
}
