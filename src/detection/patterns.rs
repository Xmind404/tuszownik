
 // ============================================================
 // DANE OSOBOWE
 // ============================================================

pub const PESEL: &str = r"\b\d{11}\b";

pub const IMIE: &str =
    r"(?i)\bimi[eę]\s*[:\-]\s*[A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+(?:[- ][A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+)*";

pub const NAZWISKO: &str =
    r"(?i)\bnazwisko\s*[:\-]\s*[A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+(?:[- ][A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+)*";

pub const DRUGIE_IMIE: &str =
    r"(?i)\bdrugie\s+imi[eę]\s*[:\-]\s*[A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+(?:[- ][A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+)*";

pub const NAZWISKO_RODOWE: &str =
    r"(?i)\bnazwisko\s+rodowe\s*[:\-]\s*[A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+(?:[- ][A-ZĄĆĘŁŃÓŚŹŻ][a-ząćęłńóśźż]+)*";

pub const DATA_URODZENIA: &str =
    r"(?i)\b(?:data\s+urodzenia|urodzony|urodzona)\s*[:\-]?\s*\d{2}[./-]\d{2}[./-]\d{4}\b";


// ============================================================
// DOKUMENTY TOŻSAMOŚCI
// ============================================================

pub const DOWOD_OSOBISTY: &str =
    r"\b[A-Z]{3}\d{6}\b";

pub const PASZPORT: &str =
    r"\b[A-Z]{2}\d{7}\b";

pub const PRAWO_JAZDY: &str =
    r"(?i)\bnumer\s+prawa\s+jazdy\s*[:\-]\s*[A-Z0-9]*\d[A-Z0-9]*\b";


// ============================================================
// IDENTYFIKATORY PRZEDSIĘBIORSTW
// ============================================================

pub const NIP: &str =
    r"(?i)\bNIP\s*[:\-]?\s*\d{10}\b";

pub const REGON_9: &str =
    r"(?i)\bREGON\s*[:\-]?\s*\d{9}\b";

pub const REGON_14: &str =
    r"(?i)\bREGON\s*[:\-]?\s*\d{14}\b";

pub const KRS: &str =
    r"(?i)\bKRS\s*[:\-]?\s*\d{10}\b";

pub const VAT_UE_PL: &str =
    r"(?i)\bPL\d{10}\b";


// ============================================================
// DANE KONTAKTOWE
// ============================================================

pub const EMAIL: &str =
    r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b";

pub const TELEFON: &str =
    r"(?i)\b(?:telefon|tel\.?|komórkowy|komorkowy)\s*[:\-]?\s*(?:\+48[\s-]?)?\d{3}(?:[\s-]?\d{3}){2}\b";

pub const KOD_POCZTOWY: &str =
    r"\b\d{2}-\d{3}\b";

pub const ADRES: &str =
    r"(?i)\b(?:ulica|ul\.|aleja|al\.|plac|pl\.)\s+[A-ZĄĆĘŁŃÓŚŹŻ][\wąćęłńóśźż.-]*(?:\s+[A-ZĄĆĘŁŃÓŚŹŻ][\wąćęłńóśźż.-]*){0,3}\s+\d+[A-Za-z]?(?:/\d+)?\b";


// ============================================================
// DANE FINANSOWE
// ============================================================

pub const IBAN_PL: &str =
    r"(?i)\bPL(?:\s?\d){26}\b";

pub const NRB: &str =
    r"\b\d{26}\b";

pub const KARTA_PLATNICZA: &str =
    r"\b(?:\d[ -]?){12,18}\d\b";

pub const BIC_SWIFT: &str =
    r"\b[A-Z]{4}PL[A-Z0-9]{2}(?:[A-Z0-9]{3})?\b";


// ============================================================
// DANE SIECIOWE
// ============================================================

pub const IPV4: &str =
    r"\b(?:\d{1,3}\.){3}\d{1,3}\b";

pub const IPV6: &str =
    r"\b(?:[0-9a-fA-F]{1,4}:){2,7}[0-9a-fA-F]{1,4}\b";

pub const MAC_ADDRESS: &str =
    r"\b(?:[0-9A-Fa-f]{2}[:-]){5}[0-9A-Fa-f]{2}\b";


// ============================================================
// DANE UWIERZYTELNIAJĄCE
// ============================================================

pub const TOKEN_BEARER: &str =
    r"(?i)\bBearer\s+[A-Za-z0-9._~+/=-]+";

pub const KLUCZ_API: &str =
    r"(?i)\b(?:api[_-]?key|access[_-]?key|secret[_-]?key)\s*[:=]\s*[A-Za-z0-9._~+/=-]{8,}";

pub const HASLO: &str =
    r"(?i)\b(?:hasło|haslo|passwd)\s*[:=]\s*[^\s,;]+";

pub const KLUCZ_PRYWATNY: &str =
    r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----";


// ============================================================
// DOKUMENTY I UMOWY
// ============================================================

pub const NUMER_UMOWY: &str =
    r"(?i)\b(?:numer\s+umowy|nr\s+umowy)\s*[:#\-]?\s*[A-Z0-9/-]{4,30}\b";

pub const NUMER_FAKTURY: &str =
    r"(?i)\b(?:numer\s+faktury|nr\s+faktury)\s*[:#\-]?\s*[A-Z0-9/-]{3,30}\b";

pub const SYGNATURA_AKT: &str =
    r"(?i)\b(?:sygnatura\s+akt|sygn\.\s*akt)\s*[:\-]?\s*[A-Z0-9 /.-]{3,40}";

pub const NUMER_POLISY: &str =
    r"(?i)\b(?:numer\s+polisy|nr\s+polisy)\s*[:#\-]?\s*[A-Z0-9/-]{4,30}\b";


// ============================================================
// POZOSTAŁE IDENTYFIKATORY
// ============================================================

pub const UUID: &str =
    r"\b[0-9a-fA-F]{8}-(?:[0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}\b";

pub const IMEI: &str =
    r"(?i)\bIMEI\s*[:\-]?\s*\d{15}\b";

pub const IMSI: &str =
    r"(?i)\bIMSI\s*[:\-]?\s*\d{14,15}\b";


// ============================================================
// WSZYSTKIE WZORCE
// ============================================================

pub const PATTERNS: &[(&str, &str)] = &[
    // Dane osobowe
    ("PESEL", PESEL),
    ("IMIE", IMIE),
    ("NAZWISKO", NAZWISKO),
    ("DRUGIE_IMIE", DRUGIE_IMIE),
    ("NAZWISKO_RODOWE", NAZWISKO_RODOWE),
    ("DATA_URODZENIA", DATA_URODZENIA),

    // Dokumenty tożsamości
    ("DOWOD_OSOBISTY", DOWOD_OSOBISTY),
    ("PASZPORT", PASZPORT),
    ("PRAWO_JAZDY", PRAWO_JAZDY),

    // Identyfikatory przedsiębiorstw
    ("NIP", NIP),
    ("REGON_9", REGON_9),
    ("REGON_14", REGON_14),
    ("KRS", KRS),
    ("VAT_UE_PL", VAT_UE_PL),

    // Dane kontaktowe
    ("EMAIL", EMAIL),
    ("TELEFON", TELEFON),
    ("KOD_POCZTOWY", KOD_POCZTOWY),
    ("ADRES", ADRES),

    // Dane finansowe
    ("IBAN_PL", IBAN_PL),
    ("NRB", NRB),
    ("KARTA_PLATNICZA", KARTA_PLATNICZA),
    ("BIC_SWIFT", BIC_SWIFT),

    // Dane sieciowe
    ("IPV4", IPV4),
    ("IPV6", IPV6),
    ("MAC_ADDRESS", MAC_ADDRESS),

    // Dane uwierzytelniające
    ("TOKEN_BEARER", TOKEN_BEARER),
    ("KLUCZ_API", KLUCZ_API),
    ("HASLO", HASLO),
    ("KLUCZ_PRYWATNY", KLUCZ_PRYWATNY),

    // Dokumenty i umowy
    ("NUMER_UMOWY", NUMER_UMOWY),
    ("NUMER_FAKTURY", NUMER_FAKTURY),
    ("SYGNATURA_AKT", SYGNATURA_AKT),
    ("NUMER_POLISY", NUMER_POLISY),

    // Pozostałe identyfikatory
    ("UUID", UUID),
    ("IMEI", IMEI),
    ("IMSI", IMSI),
];
