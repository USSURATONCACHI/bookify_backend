#![allow(deprecated)]
//! Every field should be in accordance to the following documentation:
//!
//! http://www.rusmarc.info/2017/rusmarc/fields.htm

mod fields0xx;
mod fields1xx;

use std::any::Any;

pub use fields0xx::*;
pub use fields1xx::*;

pub trait TypedField {
    fn field_number(&self) -> i128;
}

pub trait AnyTypedField: Any + TypedField {}

/// Typed collection of fields, in accordance to RUSMARC documentation.
#[rustfmt::skip]
pub struct TypedRecord {
    pub fields: Vec<Box<dyn AnyTypedField>>,
    // --- Fields 0xx
    /// 001 ИДЕНТИФИКАТОР ЗАПИСИ
    pub f001_record_id:                     Vec<Field001RecordId>,
    /// 003 ПОСТОЯННЫЙ ИДЕНТИФИКАТОР ЗАПИСИ
    pub f003_constant_record_id:            Vec<Field003PersistentRecordId>,
    pub f005_version:                       Vec<Field005Version>,
    pub f010_isbn:                          Vec<Field010Isbn>,
    pub f011_issn:                          Vec<Field011Issn>,
    pub f012_fingerprint:                   Vec<Field012Fingerprint>,
    pub f013_ismn:                          Vec<Field013Ismn>,
    pub f014_article_id:                    Vec<Field014ArticleId>,
    pub f015_isrn:                          Vec<Field015Isrn>,
    pub f016_isrc:                          Vec<Field016Isrc>,
    pub f017_other_standard_id:             Vec<Field017OtherStandardId>,
    pub f020_national_bibliography_number:  Vec<Field020NationalBibliographyNumber>,
    pub f021_state_registration_number:     Vec<Field021StateRegistrationNumber>,
    pub f022_government_publication_number: Vec<Field022GovernmentPublicationNumber>,
    pub f029_document_number:               Vec<Field029DocumentNumber>,
    pub f033_persistent_id:                 Vec<Field033PersistentId>,
    pub f035_other_system_numbers:          Vec<Field035OtherSystemNumbers>,
    pub f036_musical_incipit:               Vec<Field036MusicalIncipit>,
    pub f039_patent_application_number:     Vec<Field039PatentApplicationNumber>,
    pub f071_publisher_number:              Vec<Field071PublisherNumber>,
    pub f073_ean:                           Vec<Field073Ean>,
    #[deprecated]
    pub f079_publisher_numbers_deprecated:  Vec<Field079PublisherNumbers>,

    // --- Fields 1xx
    pub f100_general_processing_data: Vec<Field100GeneralProcessingData>,
    pub f101_language:                Vec<Field101Language>,
    pub f102_country_of_publication:  Vec<Field102CountryOfPublication>,
    pub f105_text_materials:          Vec<Field105TextMaterials>,
    pub f106_document_form:           Vec<Field106DocumentForm>,
    // TODO: Field 110
    // TODO: Field 115
    // TODO: Field 116
    // TODO: Field 117
    // TODO: Field 120
    // TODO: Field 
    // TODO: Field 
}
