use lazy_static::lazy_static;

struct Translation<'a> {
    lang: &'a str,
    left: &'a str,
    right: &'a str,
}

lazy_static! {
    static ref TRANSLATIONS: Vec<Translation<'static>> =  {
        fn parse_translation<'a>(s: &'a str) -> Translation<'a> {
            let (lang, left_and_right) = s.split_once('\t').expect("exit_code_translations is corrupt");
            let (left, right) = left_and_right.split_once('\t').expect("exit_code_translations is corrupt");
            Translation { lang, left, right }
        }
        include_str!("exit_code_translations").lines().map(parse_translation).collect()
    };
}

pub fn extract_exit_code(footer: &str) -> Option<(&'static str, i32)> {
    for translation in TRANSLATIONS.iter() {
        if let Some(matched) = extract_translation(footer, translation) {
            return Some(matched);
        }
    }
    return None;
}

fn extract_translation(s: &str, translation: &Translation<'static>) -> Option<(&'static str, i32)> {
    let (_, rest) = s.split_once(translation.left)?;
    let (num_str, _) = rest.split_once(translation.right)?;
    let exit_code = num_str.parse::<i32>().ok()?;
    Some((translation.lang, exit_code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages() {
        let (lang, code) = extract_exit_code("Process exited with code -1.").unwrap();
        assert_eq!(code, -1);
        assert_eq!(lang, "en");
        let (lang, code) = extract_exit_code("Process crashed with exitcode -1073740791.").unwrap();
        assert_eq!(code, -1073740791);
        assert_eq!(lang, "en");
        let (lang, code) = extract_exit_code("Der Prozess wurde mit Status 1 beendet.").unwrap();
        assert_eq!(code, 1);
        assert_eq!(lang, "de");
        let (lang, code) = extract_exit_code("Le processus s'est arrêté avec le code de sortie 2.").unwrap();
        assert_eq!(code, 2);
        assert_eq!(lang, "fr");
        let (lang, code) = extract_exit_code("Процесс завершился с кодом 2.").unwrap();
        assert_eq!(code, 2);
        assert_eq!(lang, "ru");
    }
}