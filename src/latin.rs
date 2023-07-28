
pub fn convert_word(word: &mut String) {
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 
        'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',    
    ];
    let vowels = ['a', 'e', 'i', 'o', 'u',];
    
    let mut c = word.chars();
    let mut mod_word = String::new();

    if let Some(l) = c.next() {
        let is_vowel = check_letter(&vowels, &l, &word);
        if is_vowel {
            mod_word = format!("{}-hay", &word);
        } else {
            let first_l = word.remove(0);
            mod_word = format!("{}-{}ay", word, first_l);
        }
        print!("modified word: {}\n", mod_word);
    };

    fn check_letter(letters: &[char], l: &char, word: &String) -> bool {
        let mut exist = false;

        for letter in letters {
            match word.chars().position(|_| l == letter) {
                Some(_) => {
                    exist = true;
                    break;
                },
                None => continue,
            };
        };

        exist
    }
}