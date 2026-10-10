use miette::Diagnostic;
use roxmltree::{Document, Node, StringStorage};
use thiserror::Error;

const AYAHS: usize = 6236;

/// A Tanzil text type, embedded verbatim from `data/tanzil/`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum Text {
    /// Uthmani script, as in the Madina Mushaf.
    #[default]
    Uthmani,
    /// Simple script without diacritics; about a third of the tokens.
    SimpleClean,
}

impl Text {
    /// The embedded Tanzil XML file.
    #[must_use]
    pub fn source(self) -> &'static str {
        match self {
            Self::Uthmani => include_str!("../data/tanzil/quran-uthmani.xml"),
            Self::SimpleClean => include_str!("../data/tanzil/quran-simple-clean.xml"),
        }
    }

    /// The Tanzil copyright block of the embedded file, verbatim.
    #[must_use]
    pub fn notice(self) -> &'static str {
        let source = self.source();
        let start = source.find("<!--").map_or(0, |i| i + "<!--".len());
        let end = source[start..].find("-->").map_or(start, |i| start + i);
        source[start..end].trim()
    }
}

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(quran::data))]
pub enum DataError {
    #[error("embedded text is not valid XML")]
    Xml(#[from] roxmltree::Error),
    #[error("embedded text is malformed: {0}")]
    Malformed(String),
    #[error("embedded text has {surahs} surahs and {ayahs} ayahs, expected 114 and 6236")]
    Counts { surahs: usize, ayahs: usize },
}

#[derive(Debug)]
struct Surah {
    basmala: Option<&'static str>,
    ayahs: Vec<&'static str>,
}

/// The Quranic text, borrowed from the embedded source.
#[derive(Debug)]
pub struct Quran {
    surahs: Vec<Surah>,
}

impl Quran {
    pub const SURAHS: u16 = 114;

    /// Parses the embedded `text` and checks its structure and counts.
    ///
    /// # Errors
    ///
    /// Fails if the XML is invalid, a surah or ayah is out of sequence, an
    /// attribute is not a verbatim slice of the source, or the counts are not
    /// 114 surahs and 6,236 ayahs.
    pub fn load(text: Text) -> Result<Self, DataError> {
        let document = Document::parse(text.source())?;
        let surahs = elements(document.root_element(), "sura")
            .enumerate()
            .map(|(i, node)| surah(node, i + 1))
            .collect::<Result<Vec<_>, _>>()?;
        let ayahs = surahs.iter().map(|s| s.ayahs.len()).sum();
        if surahs.len() != usize::from(Self::SURAHS) || ayahs != AYAHS {
            return Err(DataError::Counts {
                surahs: surahs.len(),
                ayahs,
            });
        }
        Ok(Self { surahs })
    }

    /// The number of ayahs in `surah` (1 to 114).
    #[must_use]
    pub fn ayah_count(&self, surah: u16) -> u16 {
        u16::try_from(self.surah(surah).ayahs.len()).expect("a surah has at most 286 ayahs")
    }

    /// The Basmala that opens `surah`, if the source gives one separately.
    #[must_use]
    pub fn basmala(&self, surah: u16) -> Option<&'static str> {
        self.surah(surah).basmala
    }

    /// The text of ayah `surah:ayah`, without a Basmala.
    #[must_use]
    pub fn ayah(&self, surah: u16, ayah: u16) -> &'static str {
        self.surah(surah).ayahs[usize::from(ayah) - 1]
    }

    fn surah(&self, number: u16) -> &Surah {
        &self.surahs[usize::from(number) - 1]
    }
}

fn elements<'a, 'input>(
    parent: Node<'a, 'input>,
    name: &'static str,
) -> impl Iterator<Item = Node<'a, 'input>> {
    parent
        .children()
        .filter(move |node| node.has_tag_name(name))
}

fn surah(node: Node<'_, 'static>, number: usize) -> Result<Surah, DataError> {
    check_index(node, number)?;
    let ayahs: Vec<Node<'_, 'static>> = elements(node, "aya").collect();
    let mut texts = Vec::with_capacity(ayahs.len());
    for (i, ayah) in ayahs.iter().enumerate() {
        check_index(*ayah, i + 1)?;
        texts.push(attribute(*ayah, "text")?.ok_or_else(|| missing(*ayah, "text"))?);
    }
    let basmala = match ayahs.first() {
        Some(first) => attribute(*first, "bismillah")?,
        None => None,
    };
    Ok(Surah {
        basmala,
        ayahs: texts,
    })
}

fn check_index(node: Node<'_, '_>, expected: usize) -> Result<(), DataError> {
    if node.attribute("index") == Some(expected.to_string().as_str()) {
        Ok(())
    } else {
        Err(DataError::Malformed(format!(
            "expected <{} index=\"{expected}\"> at byte {}",
            node.tag_name().name(),
            node.range().start
        )))
    }
}

fn attribute(node: Node<'_, 'static>, name: &str) -> Result<Option<&'static str>, DataError> {
    match node.attribute_node(name).map(|a| a.value_storage().clone()) {
        None => Ok(None),
        Some(StringStorage::Borrowed(value)) => Ok(Some(value)),
        Some(StringStorage::Owned(_)) => Err(DataError::Malformed(format!(
            "attribute `{name}` at byte {} is not verbatim",
            node.range().start
        ))),
    }
}

fn missing(node: Node<'_, '_>, name: &str) -> DataError {
    DataError::Malformed(format!(
        "missing attribute `{name}` at byte {}",
        node.range().start
    ))
}
