use std::num::ParseIntError;

use rusmarc_raw_macros::TypedField;

use crate::field::FieldData;
use crate::typed_record::{ParseTypedFieldError, TypedField};

use super::util::{concat_subfields, expect_max_one_subfield};

/// 001 ИДЕНТИФИКАТОР ЗАПИСИ
#[derive(Debug, TypedField)]
pub struct Field001RecordId {
    pub id: String,
}
impl From<String> for Field001RecordId {
    fn from(value: String) -> Self {
        Self { id: value }
    }
}
impl TryFrom<FieldData> for Field001RecordId {
    type Error = ParseTypedFieldError;

    fn try_from(value: FieldData) -> Result<Self, Self::Error> {
        Ok(value
            .as_singular_text()
            .ok_or("Record ID #001 cannot have subfields".to_string())?
            .to_string()
            .into())
    }
}

/// 003 ПОСТОЯННЫЙ ИДЕНТИФИКАТОР ЗАПИСИ
#[derive(Debug, TypedField)]
pub struct Field003PersistentRecordId {
    pub id: String,
}
impl From<String> for Field003PersistentRecordId {
    fn from(value: String) -> Self {
        Self { id: value }
    }
}
impl TryFrom<FieldData> for Field003PersistentRecordId {
    type Error = ParseTypedFieldError;

    fn try_from(value: FieldData) -> Result<Self, Self::Error> {
        Ok(value
            .as_singular_text()
            .ok_or("Record ID #003 cannot have subfields".to_string())?
            .to_string()
            .into())
    }
}

/// 005 ИДЕНТИФИКАТОР ВЕРСИИ
///
/// `ГГГГММДДЧЧММСС.Т` (eng: `yyyymmddHHMMSS.T`)
#[derive(Debug, TypedField)]
pub struct Field005Version {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub t: u32, // idk wtf is that T, it is not clear
}

impl Default for Field005Version {
    fn default() -> Self {
        Self {
            year: Default::default(),
            month: Default::default(),
            day: Default::default(),
            hour: Default::default(),
            minute: Default::default(),
            second: Default::default(),
            t: Default::default(),
        }
    }
}

impl TryFrom<FieldData> for Field005Version {
    type Error = ParseTypedFieldError;

    fn try_from(data: FieldData) -> Result<Self, ParseTypedFieldError> {
        let data = data
            .as_singular_text()
            .ok_or("Version cannot have subfields".to_string())?;

        Self::try_from(data)
    }
}

impl TryFrom<&str> for Field005Version {
    type Error = ParseTypedFieldError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.chars().any(|c| !c.is_digit(10) && c != '.') || !value.is_ascii() {
            return Err("Contains characters besides digits and points".to_string());
        }

        let split: Vec<&str> = value.split('.').collect();
        if split.len() > 2 {
            return Err("Value has multiple points".to_string());
        }
        let base = split[0];
        let t_text = split[1];

        // Start parsing

        let mut result = Field005Version::default();

        // Parse t
        if t_text.len() > 0 {
            result.t = t_text
                .parse()
                .map_err(|e: ParseIntError| -> String { e.to_string() })?;
        }

        // Parse year
        // [name, digits, reference]
        let parts: [(&str, usize, &mut u16); 6] = [
            ("year", 4, &mut result.year),
            ("month", 2, &mut result.month),
            ("day", 2, &mut result.day),
            ("hour", 2, &mut result.hour),
            ("minute", 2, &mut result.minute),
            ("second", 2, &mut result.second),
        ];

        let mut base = base;
        for (name, digits_count, store_to) in parts {
            if base.len() == 0 {
                break;
            }
            if value.len() < digits_count {
                return Err(format!("No {} specified", name));
            }
            let part;
            (part, base) = base.split_at(digits_count);

            *store_to = part
                .parse::<u16>()
                .map_err(|err| format!("Failed to parse {} from '{}': {:?}", name, part, err))?;
        }

        return Ok(result);
    }
}

/// 010 МЕЖДУНАРОДНЫЙ СТАНДАРТНЫЙ НОМЕР КНИГИ (ISBN)
#[derive(Debug, TypedField)]
pub struct Field010Isbn {
    /// $a   Номер (ISBN)
    pub isbn: String,
    ///$b   Уточнения     (П)
    pub clarifications: Option<String>,
    /// $d   Цена
    pub price: Option<String>,
    /// $z   Ошибочный ISBN     (П)
    pub errorneous_isbn: Option<String>,
    /// $9   Тираж     (П)
    pub circulation: Option<String>,
}

impl TryFrom<FieldData> for Field010Isbn {
    type Error = ParseTypedFieldError;

    fn try_from(data: FieldData) -> Result<Self, Self::Error> {
        // Just subfields
        let isbn = data.get_subfields('a');
        let clarifications = data.get_subfields('b');
        let price = data.get_subfields('d');
        let error_isbn = data.get_subfields('z');
        let circulation = data.get_subfields('9');

        // One string per subfield
        let isbn = expect_max_one_subfield(isbn)?;
        let error_isbn = expect_max_one_subfield(error_isbn)?;
        let price = expect_max_one_subfield(price)?;

        let clarifications = concat_subfields(clarifications);
        let circulation = concat_subfields(circulation);

        // Parse
        if isbn.is_none() {
            return Err("No ISBN specified".to_string());
        }
        let isbn = isbn.unwrap();

        // Result
        Ok(Self {
            isbn: isbn.to_owned(),
            clarifications,
            price: price.map(|x| x.to_owned()),
            errorneous_isbn: error_isbn.map(|x| x.to_owned()),
            circulation: circulation,
        })
    }
}

/// 011 МЕЖДУНАРОДНЫЙ СТАНДАРТНЫЙ НОМЕР СЕРИАЛЬНОГО ИЗДАНИЯ (ISSN)
#[derive(Debug, TypedField)]
pub struct Field011Issn {
    /// $a   Номер (ISSN)
    pub issn: Option<String>,
    /// $b   Уточнения
    pub clarifications: Option<String>,
    /// $d   Цена     (П)
    pub price: Option<String>,
    /// $f   ISSN-L, или «связывающий ISSN»
    pub issn_l: Option<String>,
    /// $g   Отмененный ISSN-L     (П)
    pub cancelled_issn_l: Option<String>,
    /// $y   Отмененный ISSN     (П)
    pub canelled_issn: Option<String>,
    /// $z   Ошибочный ISSN или ISSN-L     (П)
    pub errorneous_issn: Option<String>,
    /// $9   Тираж     (П)
    pub circulation: Option<String>,
}

/// 012 ИДЕНТИФИКАТОР ФИНГЕРПРИНТ     (П)
#[derive(Debug, TypedField)]
pub struct Field012Fingerprint {
    /// $a   Фингерпринт
    pub fingerprint: Option<String>,
    /// $2   Системный код Фингерпринт
    pub system_code_fingerprint: Option<String>,
    /// $5   Организация и экземпляр, к которому относится поле
    pub organization_and_instance: Option<String>,
    /// $9   Инвентарный номер экземпляра
    pub instance_inventory_number: Option<String>,
}

/// 013 МЕЖДУНАРОДНЫЙ СТАНДАРТНЫЙ НОМЕР ИЗДАНИЯ МУЗЫКАЛЬНОГО ПРОИЗВЕДЕНИЯ (ISMN)     (П)
#[derive(Debug, TypedField)]
pub struct Field013Ismn {
    /// $a   Номер (ISMN)
    pub ismn: Option<String>,
    /// $b   Уточнения
    pub clarifications: Option<String>,
    /// $d   Цена
    pub price: Option<String>,
    /// $z   Ошибочный ISMN     (П)
    pub errorneous_ismn: Option<String>,
    /// $9   Тираж     (П)
    pub circulation: Option<String>,
}

/// 014 ИДЕНТИФИКАТОР СТАТЬИ     (П)
#[derive(Debug, TypedField)]
pub struct Field014ArticleId {
    /// $a   Идентификатор статьи
    pub id: Option<String>,
    /// $z   Ошибочный идентификатор статьи      (П)
    pub errorneous_id: Option<String>,
    /// $2   Код системы
    pub system_code: Option<String>,
}

/// 015 МЕЖДУНАРОДНЫЙ СТАНДАРТНЫЙ НОМЕР ТЕХНИЧЕСКОГО ОТЧЕТА (ISRN)     (П)
#[derive(Debug, TypedField)]
pub struct Field015Isrn {
    /// $a   Номер (ISRN)
    pub isrn: Option<String>,
    /// $b   Уточнения
    pub clarifications: Option<String>,
    /// $d   Цена
    pub price: Option<String>,
    /// $z   Ошибочный ISRN     (П)
    pub erroneous_isrn: Option<String>,
    /// $9   Тираж     (П)
    pub circulation: Option<String>,
}

/// 016 МЕЖДУНАРОДНЫЙ СТАНДАРТНЫЙ НОМЕР АУДИО/ВИДЕО ЗАПИСИ (ISRC)     (П)
#[derive(Debug, TypedField)]
pub struct Field016Isrc {
    /// $a   Номер (ISRC)
    pub isrc: Option<String>,
    /// $b   Уточнения
    pub clarifications: Option<String>,
    /// $d   Цена
    pub price: Option<String>,
    /// $z   Ошибочный ISRC     (П)
    pub erroneous_isrc: Option<String>,
    /// $9   Тираж     (П)
    pub circulation: Option<String>,
}

/// 017 ДРУГОЙ СТАНДАРТНЫЙ ИДЕНТИФИКАТОР     (П)
#[derive(Debug, TypedField)]
pub struct Field017OtherStandardId {
    /// $a   Стандартный номер
    pub standard_number: Option<String>,
    /// $b   Уточнения
    pub clarifications: Option<String>,
    /// $d   Цена
    pub price: Option<String>,
    /// $z   Ошибочный номер / код     (П)
    pub erroneous_number: Option<String>,
    /// $2   Источник номера / кода
    pub source: Option<String>,
    /// $9   Тираж     (П)
    pub circulation: Option<String>,
}

/// 020 НОМЕР ДОКУМЕНТА В НАЦИОНАЛЬНОЙ БИБЛИОГРАФИИ     (П)
#[derive(Debug, TypedField)]
pub struct Field020NationalBibliographyNumber {
    /// $a   Код страны
    pub country_code: Option<String>,
    /// $b   Номер
    pub number: Option<String>,
    /// $z   Ошибочный номер     (П)
    pub erroneous_number: Option<String>,
    /// $9   Основное заглавие издания Российской книжной палаты
    pub main_title: Option<String>,
}

/// 021 НОМЕР ГОСУДАРСТВЕННОЙ РЕГИСТРАЦИИ     (П)
#[derive(Debug, TypedField)]
pub struct Field021StateRegistrationNumber {
    /// $a   Код страны
    pub country_code: Option<String>,
    /// $b   Номер
    pub number: Option<String>,
    /// $z   Ошибочный номер государственной регистрации     (П)
    pub erroneous_number: Option<String>,
    /// $9   Номер Листа государственной регистрации
    pub registration_sheet_number: Option<String>,
}

/// 022 НОМЕР ПУБЛИКАЦИИ ОРГАНА ГОСУДАРСТВЕННОЙ ВЛАСТИ     (П)
#[derive(Debug, TypedField)]
pub struct Field022GovernmentPublicationNumber {
    /// $a   Код страны
    pub country_code: Option<String>,
    /// $b   Номер
    pub number: Option<String>,
    /// $z   Ошибочный номер     (П)
    pub erroneous_number: Option<String>,
}

/// 029 НОМЕР ДОКУМЕНТА (НОРМАТИВНЫЕ И ТЕХНИЧЕСКИЕ ДОКУМЕНТЫ. НЕОПУБЛИКОВАННЫЕ ДОКУМЕНТЫ)     (П)
#[derive(Debug, TypedField)]
pub struct Field029DocumentNumber {
    /// $a   Страна или международная организация, присвоившая номер
    pub country_or_org: Option<String>,
    /// $b   Номер     (П)
    pub number: Option<String>,
    /// $c   Тип номера документа
    pub document_number_type: Option<String>,
    /// $d   Индекс международной классификации     (П)
    pub international_classification_index: Option<String>,
    /// $f   Организация
    pub organization: Option<String>,
}

/// 033 ПОСТОЯННЫЙ ИДЕНТИФИКАТОР ЗАПИСИ ДРУГОЙ СИСТЕМЫ     (П)
#[derive(Debug, TypedField)]
pub struct Field033PersistentId {
    /// $a   Идентификатор записи
    pub record_id: Option<String>,
    /// $z   Отмененный или ошибочный постоянный идентификатор записи     (П)
    pub erroneous_or_canceled_id: Option<String>,
}

/// 035 ДРУГИЕ СИСТЕМНЫЕ НОМЕРА     (П)
#[derive(Debug, TypedField)]
pub struct Field035OtherSystemNumbers {
    /// $a   Идентификатор записи
    pub record_id: Option<String>,
    /// $z   Отмененный или ошибочный идентификатор записи     (П)
    pub erroneous_or_canceled_id: Option<String>,
}

/// 036 МУЗЫКАЛЬНЫЙ ИНЦИПИТ     (П)
#[derive(Debug, TypedField)]
pub struct Field036MusicalIncipit {
    /// $a   Номер произведения
    pub work_number: Option<String>,
    /// $b   Номер части
    pub part_number: Option<String>,
    /// $c   Номер инципита
    pub incipit_number: Option<String>,
    /// $d   Голос / инструмент
    pub voice_or_instrument: Option<String>,
    /// $e   Роль
    pub role: Option<String>,
    /// $f   Название части     (П)
    pub part_title: Option<String>,
    /// $g   Тональность или лад
    pub tonality_or_mode: Option<String>,
    /// $m   Ключ
    pub key: Option<String>,
    /// $n   Ключевой знак альтерации
    pub key_signature: Option<String>,
    /// $o   Музыкальный размер
    pub time_signature: Option<String>,
    /// $p   Музыкальная нотация
    pub musical_notation: Option<String>,
    /// $q   Комментарии (произвольный текст)      (П)
    pub comments: Option<String>,
    /// $r   Примечание в кодированной форме
    pub encoded_note: Option<String>,
    /// $t   Литературный инципит      (П)
    pub literary_incipit: Option<String>,
    /// $u   Универсальный идентификатор ресурса      (П)
    pub uri: Option<String>,
    /// $z   Язык текста      (П)
    pub text_language: Option<String>,
    /// $2   Код системы музыкальной нотации
    pub notation_system_code: Option<String>,
}

/// 039 НОМЕР ЗАЯВКИ (ПАТЕНТНЫЕ ДОКУМЕНТЫ)     (П)
#[derive(Debug, TypedField)]
pub struct Field039PatentApplicationNumber {
    /// $a   Страна
    pub country: Option<String>,
    /// $b   Номер заявки
    pub application_number: Option<String>,
    /// $c   Дата подачи заявки
    pub submission_date: Option<String>,
}

/// 071 ИЗДАТЕЛЬСКИЙ НОМЕР     (П)
#[derive(Debug, TypedField)]
pub struct Field071PublisherNumber {
    /// $a   Номер, присвоенный агентством
    pub assigned_number: Option<String>,
    /// $b   Источник
    pub source: Option<String>,
    /// $c   Уточнение
    pub clarification: Option<String>,
    /// $d   Цена
    pub price: Option<String>,
    /// $z   Ошибочный номер     (П)
    pub erroneous_number: Option<String>,
}

/// 073 МЕЖДУНАРОДНЫЙ НОМЕР ТОВАРА (EAN)     (П)
#[derive(Debug, TypedField)]
pub struct Field073Ean {
    /// $a   Стандартный номер (EAN)
    pub standard_number: Option<String>,
    /// $b   Уточнения     
    pub clarifications: Option<String>,
    /// $c   Дополнительные коды, следующие за стандартным номером / кодом
    pub additional_codes: Option<String>,
    /// $d   Условия доступности и/или цена
    pub availability_or_price: Option<String>,
    /// $z   Ошибочный номер / код     (П)
    pub erroneous_number: Option<String>,
    /// $9   Тираж     (П)
    pub circulation: Option<String>,
}

/// 079 ИЗДАТЕЛЬСКИЕ НОМЕРА (КРОМЕ ЗВУКОЗАПИСЕЙ И НОТНЫХ ИЗДАНИЙ) (устаревшее)     (П)
#[deprecated]
#[derive(Debug, TypedField)]
pub struct Field079PublisherNumbers {
    /// $a   Издательский номер, присвоенный агентством
    pub assigned_number: Option<String>,
    /// $b   Источник
    pub source: Option<String>,
    /// $d   Цена
    pub price: Option<String>,
    /// $z   Ошибочный номер     (П)
    pub erroneous_number: Option<String>,
}
