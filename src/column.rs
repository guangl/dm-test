use sqllogictest::ColumnType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamengColumnType {
    Integer,
    FloatingPoint,
    Text,
    Boolean,
    Date,
    Timestamp,
    Decimal,
    Blob,
    Any,
}

impl ColumnType for DamengColumnType {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'I' => Some(Self::Integer),
            'R' => Some(Self::FloatingPoint),
            'T' => Some(Self::Text),
            'B' => Some(Self::Boolean),
            'D' => Some(Self::Date),
            'S' => Some(Self::Timestamp),
            'N' => Some(Self::Decimal),
            'L' => Some(Self::Blob),
            _ => Some(Self::Any),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Integer => 'I',
            Self::FloatingPoint => 'R',
            Self::Text => 'T',
            Self::Boolean => 'B',
            Self::Date => 'D',
            Self::Timestamp => 'S',
            Self::Decimal => 'N',
            Self::Blob => 'L',
            Self::Any => '?',
        }
    }
}
