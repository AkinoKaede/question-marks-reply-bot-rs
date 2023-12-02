use std::collections::HashSet;

use lazy_static::lazy_static;

pub(crate) mod on_text;
pub(crate) mod on_sticker;

lazy_static! {
    static ref QUESTION_MARKS: HashSet<char> = vec!['?', '¿', '⁇', '︖', '﹖', '？', '？'
        , '\u{2753}', '\u{2754}'].into_iter().collect();

    static ref QUESTION_MARK_EMOJIS: HashSet<String> = vec!["\u{2753}".to_string()
        , "\u{2754}".to_string()].into_iter().collect();
}

trait IsQuestionMark {
    fn is_question_mark(&self) -> bool;
}

impl IsQuestionMark for char {
    fn is_question_mark(&self) -> bool {
        QUESTION_MARKS.contains(self)
    }
}

trait QuestionMarks
    where Self: Sized
{
    fn is_composed_of_question_marks(&self) -> bool;

    fn rev(&self) -> Option<Self>;
}

impl QuestionMarks for String {
    fn is_composed_of_question_marks(&self) -> bool {
        self.chars().all(|c| c.is_question_mark())
    }

    fn rev(&self) -> Option<Self> {
        if self.is_composed_of_question_marks() {
            let question_mark_count = self.chars().filter(|c| *c == '?').count();
            let upside_down_question_mark_count = self.chars().filter(|c| *c == '¿').count();
            let text_count = self.chars().count();

            if question_mark_count + upside_down_question_mark_count == text_count
                && question_mark_count > 0 && upside_down_question_mark_count > 0
            {
                let res = self.chars().filter_map(
                    |c| {
                        match c {
                            '?' => Some('¿'),
                            '¿' => Some('?'),
                            _ => None,
                        }
                    }
                ).collect();

                return Some(res);
            }
        }

        None
    }
}

trait IsComposedOfSameChar {
    fn is_composed_of_same_char(&self) -> Option<bool>;
}

impl IsComposedOfSameChar for String {
    fn is_composed_of_same_char(&self) -> Option<bool> {
        if self.len() == 0 {
            return None;
        }

        let first_char = self.chars().next().unwrap();


        Some(self.chars().filter(|c| *c == first_char).count() == self.chars().count())
    }
}