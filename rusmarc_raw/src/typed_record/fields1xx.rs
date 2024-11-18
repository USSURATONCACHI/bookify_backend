/// Тип даты
#[derive(Debug)]
pub enum DateType {
    ///  а = текущий продолжающийся ресурс
    CurrentOngoingResource,
    ///  b = продолжающийся ресурс, публикация которого прекращена
    OngoingResourceWithEndedPublication,
    ///  c = продолжающийся ресурс с неизвестным статусом
    OngoingResourceWithUnknownStatus,
    ///  d = монография, издаваемая полностью или издаваемая в течение одного календарного года
    MonographPublishedInOneYear,
    ///  e = репродуцированный документ
    ReproducedDocument,
    ///  f = монография, дата публикации которой точно неизвестна
    MonographWithUnknownPublicationDate,
    ///  g = монография, публикация которой продолжается более года
    MonographPublishedForMoreThanOneYear,
    ///  h = монография с фактической датой публикации и датой присвоения авторского права / привилегии
    MonographWithActualPublicationDateAndCopyrightDate,
    ///  i = монографии, имеющие как дату производства, так и дату реализации
    MonographWithProductionAndRealizationDate,
    ///  j = документ с точной датой публикации
    DocumentWithExactPublicationDate,
    ///  k = монография, даты издания и изготовления которой отличаются
    MonographWithDifferentEditionAndManufacturingDates,
    ///  l = крайние даты коллекции
    ExtremeDatesCollection,
    ///  u = дата(ы) публикации неизвестна(ы)
    UnknownPublicationDate,
    Other(String),
}

impl From<String> for DateType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a" => DateType::CurrentOngoingResource,
            "b" => DateType::OngoingResourceWithEndedPublication,
            "c" => DateType::OngoingResourceWithUnknownStatus,
            "d" => DateType::MonographPublishedInOneYear,
            "e" => DateType::ReproducedDocument,
            "f" => DateType::MonographWithUnknownPublicationDate,
            "g" => DateType::MonographPublishedForMoreThanOneYear,
            "h" => DateType::MonographWithActualPublicationDateAndCopyrightDate,
            "i" => DateType::MonographWithProductionAndRealizationDate,
            "j" => DateType::DocumentWithExactPublicationDate,
            "k" => DateType::MonographWithDifferentEditionAndManufacturingDates,
            "l" => DateType::ExtremeDatesCollection,
            "u" => DateType::UnknownPublicationDate,
            _ => DateType::Other(value),
        }
    }
}

/// Код целевого назначения
#[derive(Debug)]
pub enum TargetAudienceCode {
    ///  а = для юношества, общего характера
    YouthGeneral,
    ///  b = для детей дошкольного возраста, 0-5 лет
    PreschoolChildren,
    ///  c = для детей младшего возраста, 5-10 лет
    YoungerChildren,
    ///  d = для детей среднего возраста, 9-14 лет
    MiddleAgeChildren,
    ///  е = для юношества, возраст 14-20 лет
    Youth14to20,
    ///  k = для взрослых, научная
    AdultsScientific,
    ///  m = для вхрослых, общего характера
    AdultsGeneral,
    ///  u = неизвестно
    Unknown,
    Other(String),
}

impl From<String> for TargetAudienceCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a" => TargetAudienceCode::YouthGeneral,
            "b" => TargetAudienceCode::PreschoolChildren,
            "c" => TargetAudienceCode::YoungerChildren,
            "d" => TargetAudienceCode::MiddleAgeChildren,
            "e" => TargetAudienceCode::Youth14to20,
            "k" => TargetAudienceCode::AdultsScientific,
            "m" => TargetAudienceCode::AdultsGeneral,
            "u" => TargetAudienceCode::Unknown,
            _ => TargetAudienceCode::Other(value),
        }
    }
}

/// Код публикации органа государственной власти
#[derive(Debug)]
pub enum GovernmentPublicationCode {
    ///  а = федеральный / национальный
    FederalNational,
    ///  b = республика, штат / провинция
    RepublicState,
    ///  с = край, область округ (графство) / департамент
    RegionArea,
    ///  d = местный (муниципальный, городской и т.д.)
    LocalMunicipal,
    ///  е = межтерриториальный (включающий разные департаменты и правительства ниже национального уровня)
    Interterritorial,
    ///  f = межправительственный
    Intergovernmental,
    ///  g = нелегальное правительство или правительство в изгнании
    ExiledGovernment,
    ///  h = уровень не определен
    UndefinedLevel,
    ///  u = неизвестно
    Unknown,
    ///  y = неправительственная публикация
    NonGovernmentPublication,
    ///  z = другой административный уровень
    OtherLevel,
    Other(String),
}

impl From<String> for GovernmentPublicationCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a" => GovernmentPublicationCode::FederalNational,
            "b" => GovernmentPublicationCode::RepublicState,
            "c" => GovernmentPublicationCode::RegionArea,
            "d" => GovernmentPublicationCode::LocalMunicipal,
            "e" => GovernmentPublicationCode::Interterritorial,
            "f" => GovernmentPublicationCode::Intergovernmental,
            "g" => GovernmentPublicationCode::ExiledGovernment,
            "h" => GovernmentPublicationCode::UndefinedLevel,
            "u" => GovernmentPublicationCode::Unknown,
            "y" => GovernmentPublicationCode::NonGovernmentPublication,
            "z" => GovernmentPublicationCode::OtherLevel,
            _ => GovernmentPublicationCode::Other(value),
        }
    }
}

/// Код транслитерации
#[derive(Debug)]
pub enum TransliterationCode {
    ///  a = правила транслитерации ISO
    IsoRules,
    ///  b = другие правила
    OtherRules,
    ///  c = несколько схем транслитерации – ISO или другие правила
    MultipleSystems,
    ///  y = транслитерация не используется
    NoTransliteration,
    Other(String),
}

impl From<String> for TransliterationCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a" => TransliterationCode::IsoRules,
            "b" => TransliterationCode::OtherRules,
            "c" => TransliterationCode::MultipleSystems,
            "y" => TransliterationCode::NoTransliteration,
            _ => TransliterationCode::Other(value),
        }
    }
}

/// Дополнительные наборы символов
#[derive(Debug)]
pub enum CharacterSets {
    ///  01 = ISO 646, версия IRV (основной латинский набор)
    Iso646,
    ///  02 = ISO регистрация #37 (основной кириллический набор)
    Iso37,
    ///  03 = ISO 5426 (расширенный латинский набор)
    Iso5426,
    ///  04 = ISO DIS 5427 (расширенный кириллический набор)
    Iso5427,
    ///  05 = ISO 5428 (греческий набор)
    Iso5428,
    ///  06 = ISO 6438 (набор кодированных африканских символов)
    Iso6438,
    ///  07 = ISO 10586 (набор символов грузинского алфавита)
    Iso10586,
    ///  08 = ISO 8957 (набор символов иврита) таблица 1
    Iso8957Table1,
    ///  09 = ISO 8957 (набор символов иврита) таблица 2
    Iso8957Table2,
    ///  10 [Зарезервировано]
    Reserved,
    ///  11 = ISO 5426-2 (латинские символы, используемые в редких европейских языках и устаревших типографиях)
    Iso54262,
    ///  50 = ISO 10646 (Unicode, UTF-8)
    Iso10646Unicode,
    ///  79 = Code Page 866
    CodePage866,
    ///  89 = WIN 1251
    Win1251,
    ///  99 = KOI-8
    Koi8,
    Other(String),
}

impl From<String> for CharacterSets {
    fn from(value: String) -> Self {
        match value.as_str() {
            "01" => CharacterSets::Iso646,
            "02" => CharacterSets::Iso37,
            "03" => CharacterSets::Iso5426,
            "04" => CharacterSets::Iso5427,
            "05" => CharacterSets::Iso5428,
            "06" => CharacterSets::Iso6438,
            "07" => CharacterSets::Iso10586,
            "08" => CharacterSets::Iso8957Table1,
            "09" => CharacterSets::Iso8957Table2,
            "10" => CharacterSets::Reserved,
            "11" => CharacterSets::Iso54262,
            "50" => CharacterSets::Iso10646Unicode,
            "79" => CharacterSets::CodePage866,
            "89" => CharacterSets::Win1251,
            "99" => CharacterSets::Koi8,
            _ => CharacterSets::Other(value),
        }
    }
}

/// Графика заглавия
#[derive(Debug)]
pub enum TitleGraphics {
    ///  ba = латинская
    Latin,
    ///  ca = кириллическая
    Cyrillic,
    ///  da = японская - неопределенная графика
    JapaneseUndetermined,
    ///  db = японская - канджи
    JapaneseKanji,
    ///  dc = японская - кана
    JapaneseKana,
    ///  ea = китайская
    Chinese,
    ///  fa = арабская
    Arabic,
    ///  ga = греческая
    Greek,
    ///  ha = иврит
    Hebrew,
    ///  ia = тайская
    Thai,
    ///  ja = деванагари
    Devanagari,
    ///  ka = корейская
    Korean,
    ///  la = тамильская
    Tamil,
    ///  ma = грузинская
    Georgian,
    ///  mb = армянская
    Armenian,
    ///  zz = другая
    Other(String),
}

impl From<String> for TitleGraphics {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ba" => TitleGraphics::Latin,
            "ca" => TitleGraphics::Cyrillic,
            "da" => TitleGraphics::JapaneseUndetermined,
            "db" => TitleGraphics::JapaneseKanji,
            "dc" => TitleGraphics::JapaneseKana,
            "ea" => TitleGraphics::Chinese,
            "fa" => TitleGraphics::Arabic,
            "ga" => TitleGraphics::Greek,
            "ha" => TitleGraphics::Hebrew,
            "ia" => TitleGraphics::Thai,
            "ja" => TitleGraphics::Devanagari,
            "ka" => TitleGraphics::Korean,
            "la" => TitleGraphics::Tamil,
            "ma" => TitleGraphics::Georgian,
            "mb" => TitleGraphics::Armenian,
            "zz" => TitleGraphics::Other(value),
            _ => TitleGraphics::Other(value),
        }
    }
}

/// 100 ДАННЫЕ ОБЩЕЙ ОБРАБОТКИ
pub struct Field100GeneralProcessingData {
    ///  $a / 0-7   Дата ввода записи в файл   (M - заполнение обязательно)
    pub date_input: String,
    ///  $a / 8   Тип даты (характеризует Даты 1 и 2)   (М - заполнение обязательно)
    pub date_type: DateType,
    ///  $a / 9-12   Дата 1   (М - заполнение обязательно)
    pub date_1: String,
    ///  $a / 13-16   Дата 2   (М - заполнение обязательно в соответствии с правилами, определенными типом даты)
    pub date_2: String,
    ///  $a / 17-19   Код целевого назначения
    pub target_audience_code: Option<TargetAudienceCode>,
    ///  $a / 20   Код публикации органа государственной власти
    pub publication_code: Option<GovernmentPublicationCode>,
    ///  $a / 21   Код модифицированной записи
    pub modified_record_code: Option<char>,
    ///  $a / 22-24   Язык каталогизации
    pub cataloging_language: Option<String>,
    ///  $a / 25   Код транслитерации
    pub transliteration_code: Option<TransliterationCode>,
    ///  $a / 26-29   Наборы символов   (M - заполнение обязательно)
    pub character_sets: CharacterSets,
    ///  $a / 30-33   Дополнительные наборы символов
    pub extra_character_sets: Option<CharacterSets>,
    ///  $a / 34-35   Графика заглавия
    pub title_graphics: Option<TitleGraphics>,
}

/// 101 ЯЗЫК ДОКУМЕНТА
pub struct Field101Language {
    /// $a   Язык текста, звукозаписи и т.д.
    pub language_text: Option<String>,
    /// $b   Язык промежуточного перевода
    pub language_intermediate_translation: Option<String>,
    /// $c   Язык оригинала
    pub language_original: Option<String>,
    /// $d   Язык резюме
    pub language_summary: Option<String>,
    /// $e   Язык оглавления
    pub language_contents: Option<String>,
    /// $f   Язык титульного листа
    pub language_title_page: Option<String>,
    /// $g   Язык основного заглавия
    pub language_main_title: Option<String>,
    /// $h   Язык либретто
    pub language_libretto: Option<String>,
    /// $i   Язык сопроводительного материала
    pub language_supporting_material: Option<String>,
    /// $j   Язык субтитров
    pub language_subtitles: Option<String>,
}

/// 102 СТРАНА ПУБЛИКАЦИИ ИЛИ ПРОИЗВОДСТВА
pub struct Field102CountryOfPublication {
    /// $a   Страна публикации
    pub country_of_publication: Option<String>,
    /// $b   Место издания (не ISO)
    pub place_of_publication_non_iso: Option<String>,
    /// $c   Место издания (ISO)
    pub place_of_publication_iso: Option<String>,
    /// $2   Код системы (источник кода, отличный от ISO)
    pub code_system: Option<String>,
}

/// Коды иллюстраций
#[derive(Debug)]
pub enum IllustrationCode {
    ///  a = иллюстрации
    Illustrations,
    ///  b = карты
    Maps,
    ///  c = портреты
    Portraits,
    ///  d = морские карты
    NauticalMaps,
    ///  e = планы
    Plans,
    ///  f = вкладыши
    Inserts,
    ///  g = музыкальные произведения
    MusicalWorks,
    ///  h = факсимиле
    Facsimiles,
    ///  i = гербы
    CoatsOfArms,
    ///  j = генеалогические таблицы, схемы
    GenealogicalTables,
    ///  k = формы
    Forms,
    ///  l = образцы
    Samples,
    ///  m = звукозаписи
    SoundRecordings,
    ///  n = прозрачные пленочные материалы (transparancies)
    Transparencies,
    ///  o = украшения и орнаменты, раскрашенные буквы в рукописи
    DecorationsAndOrnaments,
    ///  y = без иллюстраций (y###)
    NoIllustrations,
    Other(String),
}

impl From<String> for IllustrationCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a" => IllustrationCode::Illustrations,
            "b" => IllustrationCode::Maps,
            "c" => IllustrationCode::Portraits,
            "d" => IllustrationCode::NauticalMaps,
            "e" => IllustrationCode::Plans,
            "f" => IllustrationCode::Inserts,
            "g" => IllustrationCode::MusicalWorks,
            "h" => IllustrationCode::Facsimiles,
            "i" => IllustrationCode::CoatsOfArms,
            "j" => IllustrationCode::GenealogicalTables,
            "k" => IllustrationCode::Forms,
            "l" => IllustrationCode::Samples,
            "m" => IllustrationCode::SoundRecordings,
            "n" => IllustrationCode::Transparencies,
            "o" => IllustrationCode::DecorationsAndOrnaments,
            "y" => IllustrationCode::NoIllustrations,
            _ => IllustrationCode::Other(value),
        }
    }
}

/// Коды формы содержания
#[derive(Debug)]
pub enum ContentFormCode {
    ///  7 = академический труд уровня ниже диссертации на соискание ученой степени доктора / кандидата наук
    PreDoctoralThesis,
    ///  а = библиографическое издание
    BibliographicPublication,
    ///  b = каталог
    Catalog,
    ///  c = указатель
    Index,
    ///  d = реферат или резюме, включая аннотации
    Abstract,
    ///  e = словарь
    Dictionary,
    ///  f = энциклопедия
    Encyclopedia,
    ///  g = справочное издание общего характера
    GeneralReferenceBook,
    ///  h = описание проекта
    ProjectDescription,
    ///  i = статистические данные
    StatisticalData,
    ///  j = учебник
    Textbook,
    ///  k = патентный документ
    PatentDocument,
    ///  l = стандарт
    Standard,
    ///  m = диссертация (оригинал)
    DissertationOriginal,
    ///  n = законы и законодательные акты
    LawsAndLegislations,
    ///  o = цифровые таблицы
    DigitalTables,
    ///  p = технический отчет
    TechnicalReport,
    ///  q = экзаменационный лист
    ExaminationSheet,
    ///  r = литературный обзор/рецензия
    LiteraryReview,
    ///  s = договоры
    Contracts,
    ///  t = карикатуры или комиксы
    CartoonsOrComics,
    ///  v = диссертация (переработанная)
    RevisedDissertation,
    ///  w = религиозные тексты
    ReligiousTexts,
    ///  z = другой тип содержания
    Other(String),
}

impl From<String> for ContentFormCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "7" => ContentFormCode::PreDoctoralThesis,
            "a" => ContentFormCode::BibliographicPublication,
            "b" => ContentFormCode::Catalog,
            "c" => ContentFormCode::Index,
            "d" => ContentFormCode::Abstract,
            "e" => ContentFormCode::Dictionary,
            "f" => ContentFormCode::Encyclopedia,
            "g" => ContentFormCode::GeneralReferenceBook,
            "h" => ContentFormCode::ProjectDescription,
            "i" => ContentFormCode::StatisticalData,
            "j" => ContentFormCode::Textbook,
            "k" => ContentFormCode::PatentDocument,
            "l" => ContentFormCode::Standard,
            "m" => ContentFormCode::DissertationOriginal,
            "n" => ContentFormCode::LawsAndLegislations,
            "o" => ContentFormCode::DigitalTables,
            "p" => ContentFormCode::TechnicalReport,
            "q" => ContentFormCode::ExaminationSheet,
            "r" => ContentFormCode::LiteraryReview,
            "s" => ContentFormCode::Contracts,
            "t" => ContentFormCode::CartoonsOrComics,
            "v" => ContentFormCode::RevisedDissertation,
            "w" => ContentFormCode::ReligiousTexts,
            "z" => ContentFormCode::Other(value),
            _ => ContentFormCode::Other(value),
        }
    }
}

/// Код конференции
#[derive(Debug)]
pub enum ConferenceCode {
    ///  0 = не является изданием, публикуемым от имени конференции
    NotConferencePublication,
    ///  1 = является изданием, публикуемым от имени конференции
    ConferencePublication,
    Other(String),
}

impl From<String> for ConferenceCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "0" => ConferenceCode::NotConferencePublication,
            "1" => ConferenceCode::ConferencePublication,
            _ => ConferenceCode::Other(value),
        }
    }
}

/// Индикатор юбилейного издания
#[derive(Debug)]
pub enum AnniversaryEditionIndicator {
    ///  0 = не является юбилейным изданием
    NotAnniversaryEdition,
    ///  1 = является юбилейным изданием
    AnniversaryEdition,
    Other(String),
}

impl From<String> for AnniversaryEditionIndicator {
    fn from(value: String) -> Self {
        match value.as_str() {
            "0" => AnniversaryEditionIndicator::NotAnniversaryEdition,
            "1" => AnniversaryEditionIndicator::AnniversaryEdition,
            _ => AnniversaryEditionIndicator::Other(value),
        }
    }
}

/// Индикатор указателя
#[derive(Debug)]
pub enum IndexIndicator {
    ///  0 = указатель отсутствует
    NoIndex,
    ///  1 = указатель имеется
    IndexPresent,
    Other(String),
}

impl From<String> for IndexIndicator {
    fn from(value: String) -> Self {
        match value.as_str() {
            "0" => IndexIndicator::NoIndex,
            "1" => IndexIndicator::IndexPresent,
            _ => IndexIndicator::Other(value),
        }
    }
}

/// Код литературного жанра
#[derive(Debug)]
pub enum LiteraryGenreCode {
    ///  a = художественная литература
    Fiction,
    ///  b = драма
    Drama,
    ///  c = очерки, эссе
    Essays,
    ///  d = юмор, сатира
    HumorAndSatire,
    ///  e = письма
    Letters,
    ///  f = короткие рассказы
    ShortStories,
    ///  g = поэтические произведения
    Poetry,
    ///  h = речи и другие риторические формы
    SpeechesAndRhetoric,
    ///  y = нелитературный текст
    NonLiteraryText,
    ///  z = смешанные и другие литературные формы
    MixedAndOtherForms,
    Other(String),
}

impl From<String> for LiteraryGenreCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a" => LiteraryGenreCode::Fiction,
            "b" => LiteraryGenreCode::Drama,
            "c" => LiteraryGenreCode::Essays,
            "d" => LiteraryGenreCode::HumorAndSatire,
            "e" => LiteraryGenreCode::Letters,
            "f" => LiteraryGenreCode::ShortStories,
            "g" => LiteraryGenreCode::Poetry,
            "h" => LiteraryGenreCode::SpeechesAndRhetoric,
            "y" => LiteraryGenreCode::NonLiteraryText,
            "z" => LiteraryGenreCode::MixedAndOtherForms,
            _ => LiteraryGenreCode::Other(value),
        }
    }
}

/// Код биографии
#[derive(Debug)]
pub enum BiographyCode {
    ///  a = автобиография
    Autobiography,
    ///  b = биография отдельного лица
    BiographyOfAnIndividual,
    ///  c = коллективная биография (например, биография семьи)
    CollectiveBiography,
    ///  d = сборник биографической информации
    BiographicalCollection,
    ///  y = не биография
    NotBiography,
    Other(String),
}

impl From<String> for BiographyCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a" => BiographyCode::Autobiography,
            "b" => BiographyCode::BiographyOfAnIndividual,
            "c" => BiographyCode::CollectiveBiography,
            "d" => BiographyCode::BiographicalCollection,
            "y" => BiographyCode::NotBiography,
            _ => BiographyCode::Other(value),
        }
    }
}

/// Код ступени высшего профессионального образования
#[derive(Debug)]
pub enum HigherEducationDegreeCode {
    ///  aa = высшая школа - неполное высшее образование
    IncompleteHigherEducation,
    ///  ab = высшая школа - бакалавр
    Bachelor,
    ///  ac = высшая школа - специалист
    Specialist,
    ///  ad = высшая школа - магистр
    Master,
    ///  au = высшая школа - неизвестно
    UnknownHigherEducation,
    ///  ba = аспирантура - кандидат наук
    PostgraduateCandidate,
    ///  ca = докторантура - доктор наук
    PostdoctoralDoctor,
    ///  zz = другое
    Other(String),
}

impl From<String> for HigherEducationDegreeCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "aa" => HigherEducationDegreeCode::IncompleteHigherEducation,
            "ab" => HigherEducationDegreeCode::Bachelor,
            "ac" => HigherEducationDegreeCode::Specialist,
            "ad" => HigherEducationDegreeCode::Master,
            "au" => HigherEducationDegreeCode::UnknownHigherEducation,
            "ba" => HigherEducationDegreeCode::PostgraduateCandidate,
            "ca" => HigherEducationDegreeCode::PostdoctoralDoctor,
            "zz" => HigherEducationDegreeCode::Other(value),
            _ => HigherEducationDegreeCode::Other(value),
        }
    }
}

/// 105 ПОЛЕ КОДИРОВАННЫХ ДАННЫХ: ТЕКСТОВЫЕ МАТЕРИАЛЫ, МОНОГРАФИЧЕСКИЕ
pub struct Field105TextMaterials {
    /// $a / 0-3   Коды иллюстраций
    pub illustration_codes: Vec<IllustrationCode>,
    /// $a / 4-7   Коды формы содержания
    pub content_form_codes: Vec<ContentFormCode>,
    /// $a / 8   Код конференции
    pub conference_code: Option<ConferenceCode>,
    /// $a / 9   Индикатор юбилейного издания
    pub anniversary_edition_indicator: Option<AnniversaryEditionIndicator>,
    /// $a / 10   Индикатор указателя
    pub index_indicator: Option<IndexIndicator>,
    /// $a / 11   Код литературного жанра
    pub literary_genre_code: Option<LiteraryGenreCode>,
    /// $a / 12   Код биографии
    pub biography_code: Option<BiographyCode>,
    /// $9   Код ступени высшего профессионального образования
    pub higher_education_degree_code: Option<HigherEducationDegreeCode>,
}
/// Форма документа: кодированные данные: обозначение носителя
#[derive(Debug)]
pub enum DocumentForm {
    /// d = крупная печать
    LargePrint,
    /// е = газетный формат
    NewspaperFormat,
    /// f = шрифты Брайля и Муна
    BrailleAndMoonFonts,
    /// g = микропечать
    Microprint,
    /// h = рукописный
    Handwritten,
    /// i = информация на нескольких носителях (например, печатный материал + микрофиша)
    MultiMedia,
    /// j = минипечать
    MiniPrint,
    /// r = обычная печать
    RegularPrint,
    /// s = электронный ресурс
    ElectronicResource,
    /// t = микроформа
    Microform,
    /// z = другие формы шрифтов
    OtherFontForms,
    /// Catch-all for unknown or unsupported values
    Other(String),
}

impl From<String> for DocumentForm {
    fn from(value: String) -> Self {
        match value.as_str() {
            "d" => DocumentForm::LargePrint,
            "e" => DocumentForm::NewspaperFormat,
            "f" => DocumentForm::BrailleAndMoonFonts,
            "g" => DocumentForm::Microprint,
            "h" => DocumentForm::Handwritten,
            "i" => DocumentForm::MultiMedia,
            "j" => DocumentForm::MiniPrint,
            "r" => DocumentForm::RegularPrint,
            "s" => DocumentForm::ElectronicResource,
            "t" => DocumentForm::Microform,
            "z" => DocumentForm::OtherFontForms,
            _ => DocumentForm::Other(value),
        }
    }
}

/// 106 ПОЛЕ КОДИРОВАННЫХ ДАННЫХ: ФОРМА ДОКУМЕНТА
pub struct Field106DocumentForm {
    /// $a   Форма документа: кодированные данные: обозначение носителя
    pub document_form: Option<DocumentForm>,
}
