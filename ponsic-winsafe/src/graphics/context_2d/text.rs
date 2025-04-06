use winapi::{
    shared::windef::HFONT,
    um::{wingdi::*, winuser::*},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DrawTextMode {
    Bottom,
    CalcRect,
    Center,
    EditControl,
    EndEllipsis,
    ExpandTabs,
    ExternalLeading,
    HidePrefix,
    Internal,
    Left,
    NoCilp,
    NoFullWidthCharBreak,
    NoPrefix,
    PathEllipsis,
    PrefixOnly,
    Right,
    RtlReading,
    SingleLine,
    TabStop,
    Top,
    VCenter,
    WordBreak,
    WordEllipsis,
}

impl DrawTextMode {
    pub(crate) fn into_sys(self) -> u32 {
        match self {
            DrawTextMode::Bottom => DT_BOTTOM,
            DrawTextMode::CalcRect => DT_CALCRECT,
            DrawTextMode::Center => DT_CENTER,
            DrawTextMode::EditControl => DT_EDITCONTROL,
            DrawTextMode::EndEllipsis => DT_END_ELLIPSIS,
            DrawTextMode::ExpandTabs => DT_EXPANDTABS,
            DrawTextMode::ExternalLeading => DT_EXTERNALLEADING,
            DrawTextMode::HidePrefix => DT_HIDEPREFIX,
            DrawTextMode::Internal => DT_INTERNAL,
            DrawTextMode::Left => DT_LEFT,
            DrawTextMode::NoCilp => DT_NOCLIP,
            DrawTextMode::NoFullWidthCharBreak => DT_NOFULLWIDTHCHARBREAK,
            DrawTextMode::NoPrefix => DT_NOPREFIX,
            DrawTextMode::PathEllipsis => DT_PATH_ELLIPSIS,
            DrawTextMode::PrefixOnly => DT_PREFIXONLY,
            DrawTextMode::Right => DT_RIGHT,
            DrawTextMode::RtlReading => DT_RTLREADING,
            DrawTextMode::SingleLine => DT_SINGLELINE,
            DrawTextMode::TabStop => DT_TABSTOP,
            DrawTextMode::Top => DT_TOP,
            DrawTextMode::VCenter => DT_VCENTER,
            DrawTextMode::WordBreak => DT_WORDBREAK,
            DrawTextMode::WordEllipsis => DT_WORD_ELLIPSIS,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Font {
    font: HFONT,
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { DeleteObject(self.font as _) };
    }
}

impl Font {
    pub unsafe fn handle(&self) -> HFONT {
        self.font
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum FontWeight {
    Dontcare = 0,
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    #[default]
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Heavy = 900,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum CharSet {
    Ansi = ANSI_CHARSET,
    Baltic = BALTIC_CHARSET,
    ChineseBig5 = CHINESEBIG5_CHARSET,
    #[default]
    Default = DEFAULT_CHARSET,
    Easteurope = EASTEUROPE_CHARSET,
    Gb2312 = GB2312_CHARSET,
    Greek = GREEK_CHARSET,
    Hangul = HANGUL_CHARSET,
    Mac = MAC_CHARSET,
    Oem = OEM_CHARSET,
    Russian = RUSSIAN_CHARSET,
    Shiftjis = SHIFTJIS_CHARSET,
    Symbol = SYMBOL_CHARSET,
    Turkish = TURKISH_CHARSET,
    Vietnamese = VIETNAMESE_CHARSET,
    Johab = JOHAB_CHARSET,
    Arabic = ARABIC_CHARSET,
    Hebrew = HEBREW_CHARSET,
    Thai = THAI_CHARSET,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum OutPrecision {
    Character = OUT_CHARACTER_PRECIS,
    #[default]
    Default = OUT_DEFAULT_PRECIS,
    Device = OUT_DEVICE_PRECIS,
    Outline = OUT_OUTLINE_PRECIS,
    PostScriptOnly = OUT_PS_ONLY_PRECIS,
    Raster = OUT_RASTER_PRECIS,
    String = OUT_STRING_PRECIS,
    Stroke = OUT_STROKE_PRECIS,
    TrueTypeOnly = OUT_TT_ONLY_PRECIS,
    TrueType = OUT_TT_PRECIS,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ClipPrecision {
    Character = CLIP_CHARACTER_PRECIS,
    #[default]
    Default = CLIP_DEFAULT_PRECIS,
    DfaDisable = CLIP_DFA_DISABLE,
    Embedded = CLIP_EMBEDDED,
    LhAngles = CLIP_LH_ANGLES,
    Mask = CLIP_MASK,
    Stroke = CLIP_STROKE_PRECIS,
    TtAlways = CLIP_TT_ALWAYS,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Quality {
    Antialiased = ANTIALIASED_QUALITY,
    Cleartype = CLEARTYPE_QUALITY,
    #[default]
    Default = DEFAULT_QUALITY,
    Draft = DRAFT_QUALITY,
    NonAntialiased = NONANTIALIASED_QUALITY,
    Proof = PROOF_QUALITY,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Pitch {
    #[default]
    Default = DEFAULT_PITCH,
    Fixed = FIXED_PITCH,
    Variable = VARIABLE_PITCH,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum FontFamily {
    Decorative = FF_DECORATIVE,
    #[default]
    Dontcare = FF_DONTCARE,
    Modern = FF_MODERN,
    Roman = FF_ROMAN,
    Script = FF_SCRIPT,
    Swiss = FF_SWISS,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenFont {
    pub height: i32,
    pub widht: i32,
    pub escapement: i32,
    pub orientation: i32,
    pub weight: FontWeight,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
    pub charset: CharSet,
    pub outprecision: OutPrecision,
    pub clipprecision: ClipPrecision,
    pub quality: Quality,
    pub pitch: Pitch,
    pub family: FontFamily,
    pub name: String,
}

impl Default for GenFont {
    fn default() -> Self {
        Self {
            height: 0,
            widht: 0,
            escapement: 0,
            orientation: 0,
            weight: FontWeight::default(),
            italic: false,
            underline: false,
            strikeout: false,
            charset: CharSet::default(),
            outprecision: OutPrecision::default(),
            clipprecision: ClipPrecision::default(),
            quality: Quality::default(),
            pitch: Pitch::default(),
            family: FontFamily::default(),
            name: String::default(),
        }
    }
}

impl GenFont {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.widht = width;
        self
    }

    pub fn set_escapement(mut self, escapement: i32) -> Self {
        self.escapement = escapement;
        self
    }

    pub fn set_orientation(mut self, orientation: i32) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn set_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn set_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn set_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn set_strikeout(mut self, strikeout: bool) -> Self {
        self.strikeout = strikeout;
        self
    }

    pub fn set_charset(mut self, charset: CharSet) -> Self {
        self.charset = charset;
        self
    }

    pub fn set_outprecision(mut self, outprecision: OutPrecision) -> Self {
        self.outprecision = outprecision;
        self
    }

    pub fn set_clipprecision(mut self, clipprecision: ClipPrecision) -> Self {
        self.clipprecision = clipprecision;
        self
    }

    pub fn set_quality(mut self, quality: Quality) -> Self {
        self.quality = quality;
        self
    }

    pub fn set_pitch(mut self, pitch: Pitch) -> Self {
        self.pitch = pitch;
        self
    }

    pub fn set_family(mut self, family: FontFamily) -> Self {
        self.family = family;
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn create(self) -> Font {
        let ret = self.create_by_ref();
        assert!(!ret.font.is_null());
        ret
    }

    pub(crate) fn create_by_ref(&self) -> Font {
        let name: Vec<u16> = self.name.encode_utf16().chain(Some(0)).collect();
        unsafe {
            Font {
                font: CreateFontW(
                    self.height,
                    self.widht,
                    self.escapement,
                    self.orientation,
                    self.weight as _,
                    self.italic as _,
                    self.underline as _,
                    self.strikeout as _,
                    self.charset as _,
                    self.outprecision as _,
                    self.clipprecision as _,
                    self.quality as _,
                    self.pitch as u32 | self.family as u32,
                    name.as_ptr() as _,
                ),
            }
        }
    }
}
