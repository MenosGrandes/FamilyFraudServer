use core::fmt;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Topic {
    ShowAnswer,
    GetNewAnswers,
    NewAnswers,
    LifeLost,
    SetId,
    Unknown,
}
impl Default for Topic {
    fn default() -> Self {
        Topic::Unknown
    }
}
impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Topic::ShowAnswer => write!(f, "ShowAnswer"),
            Topic::GetNewAnswers => write!(f, "GetNewAnswers"),
            Topic::NewAnswers => write!(f, "NewAnswers"),
            Topic::LifeLost => write!(f, "LifeLost"),
            Topic::SetId => write!(f, "SetId"),
            Topic::Unknown => write!(f, "Unknown"),
        }
    }
}
impl TryFrom<u8> for Topic {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Topic::ShowAnswer as u8 => Ok(Topic::ShowAnswer),
            x if x == Topic::GetNewAnswers as u8 => Ok(Topic::GetNewAnswers),
            x if x == Topic::NewAnswers as u8 => Ok(Topic::NewAnswers),
            x if x == Topic::LifeLost as u8 => Ok(Topic::LifeLost),
            x if x == Topic::SetId as u8 => Ok(Topic::SetId),
            x if x == Topic::Unknown as u8 => Ok(Topic::Unknown),

            _ => Err(()),
        }
    }
}
