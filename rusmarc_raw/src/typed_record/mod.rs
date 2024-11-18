#![allow(deprecated)]
//! Every field should be in accordance to the following documentation:
//!
//! http://www.rusmarc.info/2017/rusmarc/fields.htm

mod fields0xx;
mod fields1xx;

use std::any::Any;

use crate::field::FieldData;
pub use fields0xx::*;
pub use fields1xx::*;

pub trait TypedField {
    fn field_number(&self) -> u128;
}

type ParseTypedFieldError = String;
pub trait ParseTypedField {
    fn parse(data: FieldData) -> Result<Box<dyn AnyTypedField>, ParseTypedFieldError>;
}

pub trait AnyTypedField {
    fn any_ref(&self) -> &dyn Any;
    fn any_mut(&mut self) -> &mut dyn Any;

    fn typed_field_ref(&self) -> &dyn TypedField;
    fn typed_field_mut(&mut self) -> &mut dyn TypedField;

    fn field_number(&self) -> u128 {
        self.typed_field_ref().field_number()
    }
}

impl<T: Any + TypedField> AnyTypedField for T {
    fn any_ref(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn typed_field_ref(&self) -> &dyn TypedField {
        self as &dyn TypedField
    }
    fn typed_field_mut(&mut self) -> &mut dyn TypedField {
        self as &mut dyn TypedField
    }
}

/// Typed collection of fields, in accordance to RUSMARC documentation.
pub struct TypedRecord {
    pub fields: Vec<Box<dyn AnyTypedField>>,
}

#[rustfmt::skip]
pub fn parse_typed_field(field: crate::field::Field) -> Result<Box<dyn AnyTypedField>, String> {
    match field.number {
        001 => Field001RecordId                   ::parse(field.data),
        003 => Field003PersistentRecordId         ::parse(field.data),
        // 005 => Field005Version                    ::parse(data.data),
        // 010 => Field010Isbn                       ::parse(data.data),
        // 011 => Field011Issn                       ::parse(data.data),
        // 012 => Field012Fingerprint                ::parse(data.data),
        // 013 => Field013Ismn                       ::parse(data.data),
        // 014 => Field014ArticleId                  ::parse(data.data),
        // 015 => Field015Isrn                       ::parse(data.data),
        // 016 => Field016Isrc                       ::parse(data.data),
        // 017 => Field017OtherStandardId            ::parse(data.data),
        // 020 => Field020NationalBibliographyNumber ::parse(data.data),
        // 021 => Field021StateRegistrationNumber    ::parse(data.data),
        // 022 => Field022GovernmentPublicationNumber::parse(data.data),
        // 029 => Field029DocumentNumber             ::parse(data.data),
        // 033 => Field033PersistentId               ::parse(data.data),
        // 035 => Field035OtherSystemNumbers         ::parse(data.data),
        // 036 => Field036MusicalIncipit             ::parse(data.data),
        // 039 => Field039PatentApplicationNumber    ::parse(data.data),
        // 071 => Field071PublisherNumber            ::parse(data.data),
        // 073 => Field073Ean                        ::parse(data.data),
        // 079 => Field079PublisherNumbers           ::parse(data.data),
        // 100 => Field100GeneralProcessingData      ::parse(data.data),
        // 101 => Field101Language                   ::parse(data.data),
        // 102 => Field102CountryOfPublication       ::parse(data.data),
        // 105 => Field105TextMaterials              ::parse(data.data),
        // 106 => Field106DocumentForm               ::parse(data.data),
        _ => Err("Unknown field type".to_owned()),
    }
}
