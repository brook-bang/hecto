use crate::{editor::annotatedstring::annotationtype::AnnotationType, prelude::*};

#[derive(Copy,Clone,Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation{
    pub annotation_type: AnnotationType,
    pub start: ByteIdx,
    pub end: ByteIdx,
}

