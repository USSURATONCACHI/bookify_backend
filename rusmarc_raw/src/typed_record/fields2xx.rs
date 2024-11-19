// TODO: 205 Сведения об издании
// TODO: 206 Область специфических сведений: картографические материалы - математические данные
// TODO: 207 Область специфических сведений: нумерация продолжающихся ресурсов
// TODO: 208 Область специфических сведений: нотные издания
// TODO: 211 Запланированная дата издания
// TODO: 225 Серия
// TODO: 229 Область специфических сведений: нормативно-технические и технические документы. Неопубликованные документы
// TODO: 230 Область специфических сведений: электронные ресурсы
// TODO: 239 Область специфических сведений: нормативные и технические документы
// TODO: 251 Организация и порядок расположения материалов
// TODO: 283 Тип носителя

// TODO: 203 Вид содержания и тип средства
// TODO: 210 Публикация, распространение и др.
// TODO: 215 Физическая характеристика

use crate::typed_record::TypedField;
use rusmarc_raw_macros::TypedField;

use crate::field::FieldData;

use super::{util::concat_subfields, ParseTypedFieldError};

/// 200 ЗАГЛАВИЕ И СВЕДЕНИЯ ОБ ОТВЕТСТВЕННОСТИ
#[derive(Debug, TypedField)]
pub struct Field200Header {
    /// $a   Основное заглавие     (П)
    pub main_title: Option<String>,
    /// $b   Общее обозначение материала     (П)
    pub general_material_notion: Option<String>,
    /// $d   Параллельное заглавие     (П)
    pub parallel_title: Option<String>,
    /// $e   Сведения, относящиеся к заглавию     (П)
    pub title_related_info: Option<String>,
    /// $f   Первые сведения об ответственности     (П)
    pub main_responsibility: Option<String>,
    /// $g   Последующие сведения об ответственности     (П)
    pub other_responsibility: Option<String>,
    /// $h   Номер части     (П)
    pub part_number: Option<String>,
    /// $i   Наименование части     (П)
    pub part_name: Option<String>,
    /// $j   Крайние даты
    pub end_dates: Option<String>,
    /// $k   Даты основной массы документов
    pub main_documents_dates: Option<String>,
    /// $r   Информация с титульного листа, следующая за основным заглавием (для старопечатных изданий)
    pub title_page_info: Option<String>,
    /// $v   Обозначение тома
    pub volume_notion: Option<String>,
    /// $z   Язык параллельного заглавия     (П)
    pub parallel_title_lang: Option<String>,
    /// $5   Организация и экземпляр, к которому относится поле
    pub org_and_instance: Option<String>,
}

impl TryFrom<FieldData> for Field200Header {
    type Error = ParseTypedFieldError;

    #[rustfmt::skip]
    fn try_from(value: FieldData) -> Result<Self, Self::Error> {
        // Result
        Ok(Self {
            main_title:              concat_subfields(value.get_subfields('a')),
            general_material_notion: concat_subfields(value.get_subfields('b')),
            parallel_title:          concat_subfields(value.get_subfields('d')),
            title_related_info:      concat_subfields(value.get_subfields('e')),
            main_responsibility:     concat_subfields(value.get_subfields('f')),
            other_responsibility:    concat_subfields(value.get_subfields('g')),
            part_number:             concat_subfields(value.get_subfields('h')),
            part_name:               concat_subfields(value.get_subfields('i')),
            end_dates:               concat_subfields(value.get_subfields('j')),
            main_documents_dates:    concat_subfields(value.get_subfields('k')),
            title_page_info:         concat_subfields(value.get_subfields('r')),
            volume_notion:           concat_subfields(value.get_subfields('v')),
            parallel_title_lang:     concat_subfields(value.get_subfields('z')),
            org_and_instance:        concat_subfields(value.get_subfields('5')),
        })
    }
}
