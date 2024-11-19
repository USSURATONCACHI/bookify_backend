#![allow(deprecated)]
//! Every field should be in accordance to the following documentation:
//!
//! <http://www.rusmarc.info/2017/rusmarc/fields.htm>

mod fields0xx;
mod fields1xx;
mod fields2xx;
mod util;

use std::{any::Any, marker::PhantomData};

use crate::field::FieldData;
pub use fields0xx::*;
pub use fields1xx::*;
pub use fields2xx::*;

pub trait TypedField: std::fmt::Debug {
    fn field_number(&self) -> u128;
}

pub type ParseTypedFieldError = String;
pub trait ParseTypedField {
    fn parse(data: FieldData) -> Result<Box<dyn AnyTypedField>, ParseTypedFieldError>;
}

impl<T> ParseTypedField for T
where
    T: 'static + AnyTypedField + TryFrom<FieldData, Error = ParseTypedFieldError>,
{
    fn parse(data: FieldData) -> Result<Box<dyn AnyTypedField>, ParseTypedFieldError> {
        Self::try_from(data).map(|x| Box::new(x) as _)
    }
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
#[derive(Debug)]
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
        001 => field.data.parse::<Field001RecordId>(),
        003 => field.data.parse::<Field003PersistentRecordId>(),
        005 => field.data.parse::<Field005Version>(),
        010 => field.data.parse::<Field010Isbn>(),
        200 => field.data.parse::<Field200Header>(),
        // 011 => field.data.parse::<Field011Issn>(),
        // 012 => field.data.parse::<Field012Fingerprint>(),
        // 013 => field.data.parse::<Field013Ismn>(),
        // 014 => field.data.parse::<Field014ArticleId>(),
        // 015 => field.data.parse::<Field015Isrn>(),
        // 016 => field.data.parse::<Field016Isrc>(),
        // 017 => field.data.parse::<Field017OtherStandardId>(),
        // 020 => field.data.parse::<Field020NationalBibliographyNumber>(),
        // 021 => field.data.parse::<Field021StateRegistrationNumber>(),
        // 022 => field.data.parse::<Field022GovernmentPublicationNumber>(),
        // 029 => field.data.parse::<Field029DocumentNumber>(),
        // 033 => field.data.parse::<Field033PersistentId>(),
        // 035 => field.data.parse::<Field035OtherSystemNumbers>(),
        // 036 => field.data.parse::<Field036MusicalIncipit>(),
        // 039 => field.data.parse::<Field039PatentApplicationNumber>(),
        // 071 => field.data.parse::<Field071PublisherNumber>(),
        // 073 => field.data.parse::<Field073Ean>(),
        // 079 => field.data.parse::<Field079PublisherNumbers>(),
        // 100 => field.data.parse::<Field100GeneralProcessingData>(),
        // 101 => field.data.parse::<Field101Language>(),
        // 102 => field.data.parse::<Field102CountryOfPublication>(),
        // 105 => field.data.parse::<Field105TextMaterials>(),
        // 106 => field.data.parse::<Field106DocumentForm>(),
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
