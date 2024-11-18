#![allow(deprecated)]
//! Every field should be in accordance to the following documentation:
//!
//! <http://www.rusmarc.info/2017/rusmarc/fields.htm>

mod fields0xx;
mod fields1xx;

use std::{any::Any, marker::PhantomData};

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

pub trait AnyTypedField: std::fmt::Debug {
    fn any_ref(&self) -> &dyn Any;
    fn any_mut(&mut self) -> &mut dyn Any;

    fn typed_field_ref(&self) -> &dyn TypedField;
    fn typed_field_mut(&mut self) -> &mut dyn TypedField;

    fn field_number(&self) -> u128 {
        self.typed_field_ref().field_number()
    }
}

impl<T: Any + TypedField + std::fmt::Debug> AnyTypedField for T {
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
///
/// ## General usage
/// ```
/// # use std::io::{BufReader, Cursor};
/// use rusmarc_raw::record::RecordsReader;
/// use rusmarc_raw::record::record_remove_errors;
/// use rusmarc_raw::typed_record::{TypedRecord, Field001RecordId};
///
/// // Test data
/// let data = "#1: id-001\n#2: some other field\n*****\n#1: id-002";
/// let cursor = Cursor::new(data);
/// let buf_reader = BufReader::new(cursor);
///
/// let mut iter = RecordsReader::new(buf_reader)               // Parse raw text into untyped records
///     .map(|record| record_remove_errors(record.unwrap()))    // Ignore all fields with errors
///     .map(|record| TypedRecord::parse(record.into_iter()))   // Parse untyped records into typed ones
///     .map(|(record, _errors)| record);                       // Ignore all errors from incorrectly typed fields
///
/// // Check that first record has id "id-001"
/// let record1 = iter.next().unwrap();
/// let id1 = record1.get_fields::<Field001RecordId>().next().unwrap();
/// assert_eq!(id1.id, "id-001");
///
/// // Check that second record has id "id-002"
/// let record2 = iter.next().unwrap();
/// let id2 = record2.get_fields::<Field001RecordId>().next().unwrap();
/// assert_eq!(id2.id, "id-002");
/// ```
pub struct TypedRecord {
    pub fields: Vec<Box<dyn AnyTypedField>>,
}

/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 6);
/// ```
pub struct TypedFieldsView<'a, T: 'static + TypedField> {
    items: &'a [Box<dyn AnyTypedField>],
    _type_is_needed: PhantomData<T>,
}

impl<'a, T: 'static + TypedField> TypedFieldsView<'a, T> {
    fn new(items: &'a [Box<dyn AnyTypedField>]) -> Self {
        Self {
            items,
            _type_is_needed: PhantomData::default(),
        }
    }
}

impl<'a, T: 'static + TypedField> Iterator for TypedFieldsView<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.items.len() > 0 {
            let item = &self.items[0];
            self.items = &self.items[1..];

            if let Some(x) = item.any_ref().downcast_ref::<T>() {
                return Some(x);
            }
        }

        None
    }
}

impl TypedRecord {
    pub fn parse(
        fields: impl Iterator<Item = crate::field::Field>,
    ) -> (Self, Vec<ParseTypedFieldError>) {
        let parse = fields.map(|x| parse_typed_field(x));

        let (oks, errs): (Vec<_>, Vec<_>) = parse.partition(|x| x.is_ok());
        let oks = oks.into_iter().map(|x| x.unwrap());
        let errs = errs.into_iter().map(|x| x.unwrap_err());

        (
            TypedRecord {
                fields: oks.collect(),
            },
            errs.collect(),
        )
    }

    pub fn get_fields<'a, T: 'static + TypedField>(&'a self) -> TypedFieldsView<'a, T> {
        let iter: TypedFieldsView<'a, T> = TypedFieldsView::new(&self.fields);
        iter
    }
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

/*

All fields used by Znanium, Ssga, and Lan:

001
003
005
010
011
030
046
060
062
066
090
091
092

100
101
102
105
106
110
123
125
135
181
182

200
203
205
210
215
225
230
239

300
314
320
327
328
330
331
333

400
421
454
461
488

510
541

600
601
606
607
608
610
621
675
686
690
691
692
693
694
699

700
701
702
710
711
712
740

801
852

900
903
904
905
906
907
908
909
910
912
919
920
922
923
925
926
932
933
934
935
936
940
943
951
953
961
962
964
965
971
972
982
985
990
991
998
999

1000
1040
5501
2147483647

*/
