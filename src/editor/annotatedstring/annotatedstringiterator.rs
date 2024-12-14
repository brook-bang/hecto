use super::AnnotatedString;

pub struct AnnotatedStringIterator<'a>{
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: usize,
}