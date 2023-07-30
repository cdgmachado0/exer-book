
pub fn convert_word(my_str: &str) {
    let vowels = ['a', 'e', 'i', 'o', 'u',];

    let mut word = String::from(my_str);
    
    let mut c = word.chars();
    
    #[allow(unused_assignments)]
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