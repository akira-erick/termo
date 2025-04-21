use rand::Rng;
use std::fs;

pub struct Word {
    word: String,
}
impl Word {
    fn new(word: String) -> Word {
        if !Word::validate_word_length(&word) {
            panic!("Word must be 5 characters long")
        };
        if !Word::validate_word_alphabetic(&word) {
            panic!("Word must only contain alphabetic characters without accents")
        };

        let word = word.to_lowercase();

        Word { word }
    }

    pub fn random() -> Word {
        let words = Word::get_word_list();
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..words.len());
        let random_word = words[random_index].clone();
        Word::new(random_word)
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }

    pub fn attempt(&self, attempt: String) -> [char; 5] {
        if !Word::validate_word_length(&attempt) {
            panic!("Word must be 5 characters long")
        };
        if !Word::validate_word_alphabetic(&attempt) {
            panic!("Word must only contain alphabetic characters without accents")
        };

        let attempt = attempt.to_lowercase();

        let mut result = ['R'; 5];
        let mut word_chars = self.word.chars().collect::<Vec<_>>();
        let attempt_chars = attempt.chars().collect::<Vec<_>>();

        for i in 0..5 {
            if attempt_chars[i] == word_chars[i] {
                result[i] = 'G';
                word_chars[i] = ' ';
            }
        }

        for i in 0..5 {
            if result[i] != 'G' {
                for j in word_chars.iter_mut().take(5) {
                    if attempt_chars[i] == *j {
                        result[i] = 'Y';
                        *j = ' ';
                        break;
                    }
                }
            }
        }

        result
    }

    fn validate_word_length(word: &str) -> bool {
        word.chars().count() == 5
    }

    fn validate_word_alphabetic(word: &str) -> bool {
        word.chars().all(|c| c.is_alphabetic())
    }

    fn get_word_list() -> Vec<String> {
        let data =
            fs::read_to_string("src/words_data/words_data.txt").expect("Unable to read file");
        data.lines().map(|line| line.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_creation() {
        let word = Word::new("barro".to_string());
        assert_eq!(word.get_word(), "barro");
    }

    #[test]
    fn test_all_red_letters() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("fghij".to_string());
        assert_eq!(result, ['R', 'R', 'R', 'R', 'R']);
    }

    #[test]
    fn test_all_green_letters() {
        let word = Word::new("aaaaa".to_string());
        let result = word.attempt("aaaaa".to_string());
        assert_eq!(result, ['G', 'G', 'G', 'G', 'G']);
    }

    #[test]
    fn test_one_green_letter() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("afghi".to_string());
        assert_eq!(result, ['G', 'R', 'R', 'R', 'R']);
    }

    #[test]
    fn test_all_yellow_letters() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("edbac".to_string());
        assert_eq!(result, ['Y', 'Y', 'Y', 'Y', 'Y']);
    }

    #[test]
    fn test_one_yellow_letter() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("faghi".to_string());
        assert_eq!(result, ['R', 'Y', 'R', 'R', 'R']);
    }

    #[test]
    fn test_mixed_letters() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("afghb".to_string());
        assert_eq!(result, ['G', 'R', 'R', 'R', 'Y']);
    }

    #[test]
    fn test_word_validation_length() {
        assert!(Word::validate_word_length("abcde"));
        assert!(!Word::validate_word_length("abcd"));
        assert!(!Word::validate_word_length("abcdef"));
    }

    #[test]
    fn test_word_validation_alphabetic() {
        assert!(Word::validate_word_alphabetic("abcde"));
        assert!(!Word::validate_word_alphabetic("abc1e"));
        assert!(!Word::validate_word_alphabetic("abcde!"));
    }

    #[test]
    #[should_panic(expected = "Word must be 5 characters long")]
    fn test_word_creation_too_short() {
        Word::new("bar".to_string());
    }

    #[test]
    #[should_panic(expected = "Word must only contain alphabetic characters without accents")]
    fn test_word_creation_invalid_characters() {
        Word::new("b@rro".to_string());
    }

    #[test]
    #[should_panic(expected = "Word must be 5 characters long")]
    fn test_word_attempt_too_short() {
        let word = Word::new("barro".to_string());
        word.attempt("bar".to_string());
    }

    #[test]
    #[should_panic(expected = "Word must only contain alphabetic characters without accents")]
    fn test_word_attempt_invalid_characters() {
        let word = Word::new("barro".to_string());
        word.attempt("b@rro".to_string());
    }
}
