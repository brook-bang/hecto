use std::{
    cmp::{max, min},
    fmt::{self, Display}, string,
};
pub mod annotationtype;
pub use annotationtype::AnnotationType;
mod annotation;
use annotation::Annotation;
mod annotatedstringpart;
use annotatedstringpart::AnnotatedStringPart;
mod annotatedstringiterator;
use annotatedstringiterator::AnnotatedStringIterator;
#[derive(Default,Debug)]
pub struct AnnotatedString {
    string: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
            annotations:Vec::new(),
        }
    }

    pub fn add_annotation(
        &mut self,
        annotation_type:AnnotationType,
        start_byte_idx: usize,
        end_byte_idx: usize,
    ) {
        debug_assert!(start_byte_idx <= end_byte_idx);
        self.annotations.push(Annotation {
            annotation_type,
            start_byte_idx,
            end_byte_idx,
        });
    }

    pub fn replace(
        &mut self,
        start_byte_idx: usize,
        end_byte_idx: usize,
        new_string: &str
    ) {
        debug_assert!(start_byte_idx <= end_byte_idx);
        let end_byte_idx = min(end_byte_idx,self.string.len());
        if start_byte_idx > end_byte_idx {
            return;
        }

        self.string.replace_range(start_byte_idx..end_byte_idx, new_string);

        let replaced_range_len = end_byte_idx.saturating_sub(start_byte_idx);

        let shortened = new_string.len() < replaced_range_len;

        let len_difference = new_string.len().abs_diff(replaced_range_len);

        if len_difference == 0 {
            return;
        }

        self.annotations.iter_mut().for_each(|annotation| {
            
        });


    }


    
}