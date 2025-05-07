use rand::Rng;
use std::fs;

pub struct Word {
    word: String,
}
impl Word {
    fn new(word: String) -> Word {
        Word::prepare_word(&word).expect("Invalid word");

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

    pub fn attempt(&self, attempt: &str) -> Result<[char; 5], String> {
        Word::prepare_word(&attempt)?;

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

        Ok(result)
    }

    fn validate_word_length(word: &str) -> bool {
        word.chars().count() == 5
    }

    fn validate_word_alphabetic(word: &str) -> bool {
        word.chars().all(|c| c.is_alphabetic())
    }

    fn prepare_word(word: &str) -> Result<String, String> {
        if !Word::validate_word_length(word) {
            return Err("Word must be 5 characters long".to_string());
        };
        if !Word::validate_word_alphabetic(word) {
            return Err("Word must only contain alphabetic characters without accents".to_string());
        };
        Ok(word.to_lowercase())
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
        let result = word.attempt("fghij");
        assert_eq!(result.unwrap(), ['R', 'R', 'R', 'R', 'R']);
    }

    #[test]
    fn test_all_green_letters() {
        let word = Word::new("aaaaa".to_string());
        let result = word.attempt("aaaaa");
        assert_eq!(result.unwrap(), ['G', 'G', 'G', 'G', 'G']);
    }

    #[test]
    fn test_one_green_letter() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("afghi");
        assert_eq!(result.unwrap(), ['G', 'R', 'R', 'R', 'R']);
    }

    #[test]
    fn test_all_yellow_letters() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("edbac");
        assert_eq!(result.unwrap(), ['Y', 'Y', 'Y', 'Y', 'Y']);
    }

    #[test]
    fn test_one_yellow_letter() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("faghi");
        assert_eq!(result.unwrap(), ['R', 'Y', 'R', 'R', 'R']);
    }

    #[test]
    fn test_mixed_letters() {
        let word = Word::new("abcde".to_string());
        let result = word.attempt("afghb");
        assert_eq!(result.unwrap(), ['G', 'R', 'R', 'R', 'Y']);
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
    fn test_word_preparation() {
        let word = Word::prepare_word("abCde");
        assert_eq!(word.unwrap(), "abcde");
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
    fn test_word_attempt_too_short() {
        let word = Word::new("barro".to_string());
        assert!(word.attempt("bar").is_err());
        assert_eq!(
            word.attempt("bar").unwrap_err(),
            "Word must be 5 characters long"
        );
    }

    #[test]
    fn test_word_attempt_invalid_characters() {
        let word = Word::new("barro".to_string());
        assert!(word.attempt("b@rro").is_err());
        assert_eq!(
            word.attempt("b@rro").unwrap_err(),
            "Word must only contain alphabetic characters without accents"
        );
    }
}
