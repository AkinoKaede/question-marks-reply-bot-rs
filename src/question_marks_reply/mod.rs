use std::cell::LazyCell;
use std::collections::HashSet;

pub(crate) mod on_text;
pub(crate) mod on_sticker;

const QUESTION_MARKS: LazyCell<HashSet<&char>> = LazyCell::new(||{
    ['?', '¿', '⁇', '︖', '﹖', '？', '？', '؟', '՞', '፧', '⍰'
         , '\u{2753}', '\u{2754}'].iter().collect()
});


trait IsQuestionMark {
    fn is_question_mark(&self) -> bool;
}

impl IsQuestionMark for char {
    fn is_question_mark(&self) -> bool {
        QUESTION_MARKS.contains(self)
    }
}

trait QuestionMarks
where
    Self: Sized,
{
    fn is_composed_of_question_marks(&self) -> bool;

    fn is_contains_question_marks(&self) -> bool;

    fn rev(&self) -> Option<Self>;
}

impl QuestionMarks for String {
    fn is_composed_of_question_marks(&self) -> bool {
        self.chars().all(|c| c.is_question_mark())
    }

    fn is_contains_question_marks(&self) -> bool {
        self.chars().any(|c| c.is_question_mark())
    }

    fn rev(&self) -> Option<Self> {
        if self.is_composed_of_question_marks() {
            let question_mark_count = self.chars().filter(|c| *c == '?').count();
            let upside_down_question_mark_count = self.chars().filter(|c| *c == '¿').count();
            let arabic_question_mark_count = self.chars().filter(|c| *c == '؟').count();
            let text_count = self.chars().count();

            if vec![question_mark_count, upside_down_question_mark_count, arabic_question_mark_count].iter()
                .filter(|&x| x > &0).count() == 2 {
                let (first_char, second_char) = if question_mark_count + upside_down_question_mark_count == text_count {
                    ('?', '¿')
                } else if question_mark_count + arabic_question_mark_count == text_count {
                    ('?', '؟')
                } else if upside_down_question_mark_count + arabic_question_mark_count == text_count {
                    ('¿', '؟')
                } else {
                    return None;
                };

                let res = self.chars().filter_map(
                    |c| {
                        match c {
                            c if c == first_char => Some(second_char),
                            c if c == second_char => Some(first_char),
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