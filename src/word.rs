pub struct Word{
    word: String
}
impl Word {
    pub fn new(word:String) -> Word {

        if word.len() != 5 {
            panic!("Word must be 5 characters long");
        }
        if !word.chars().all(|c| c.is_alphabetic()) {
            panic!("Word must only contain alphabetic characters without accents");
        }
        let word = word.to_lowercase();
        Word { word: word }
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }

    
    pub fn attempt(&self, attempt: String) -> [char; 5] {
        if attempt.len() != 5 {
            panic!("Word must be 5 characters long");
        }
        if !attempt.chars().all(|c| c.is_alphabetic()) {
            panic!("Word must only contain alphabetic characters without accents");
        }
        let attempt = attempt.to_lowercase();

        let mut result = [' '; 5];
        let mut word_chars = self.word.chars().collect::<Vec<_>>();
        let attempt_chars = attempt.chars().collect::<Vec<_>>();

        for i in 0..5 {
            if attempt_chars[i] == word_chars[i]{
                result[i] = 'G';
            }
        }
        for i in 0..5{
            if let Some(pos) = word_chars.iter().position(|&c| c == attempt_chars[i]) {
                if result[i] != 'G'{
                    result[i] = 'Y';
                }
                word_chars.remove(pos);
            } else {
                result[i] = 'R';
            }
        }

        result
    }

    #[allow(dead_code)]
    fn get_word_list() -> Vec<String> {
        vec![
            "barro", "banco", "bicho", "bisco", "macho",
            "omega", "arara", "livro", "viola", "frita",
            "sorte", "sabor", "carta", "corte", "festa",
            "pasta", "piano", "salsa", "torta", "tigre",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
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
    fn test_word_attempt() {
        let word = Word::new("barro".to_string());
        let result = word.attempt("baror".to_string());
        assert_eq!(result, ['G', 'G', 'G', 'Y', 'Y']);
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