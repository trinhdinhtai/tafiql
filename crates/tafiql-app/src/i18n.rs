//! UI strings via fluent (FTL).
//!
//! English + Vietnamese with English as the fallback (user story 33).
//! The FTL sources ship inside the binary via `include_str!`, but the
//! [`FluentBundle`]s are built lazily on the first lookup, so startup
//! doesn't pay for them.

use std::cell::OnceCell;

use fluent::{FluentBundle, FluentResource};
use gpui::Global;
use unic_langid::{langid, LanguageIdentifier};

const EN_FTL: &str = include_str!("../locales/en.ftl");
const VI_FTL: &str = include_str!("../locales/vi.ftl");

/// The UI languages tafiql ships with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiLanguage {
    English,
    Vietnamese,
}

impl UiLanguage {
    fn langid(self) -> LanguageIdentifier {
        match self {
            UiLanguage::English => langid!("en"),
            UiLanguage::Vietnamese => langid!("vi"),
        }
    }

    fn toggled(self) -> Self {
        match self {
            UiLanguage::English => UiLanguage::Vietnamese,
            UiLanguage::Vietnamese => UiLanguage::English,
        }
    }
}

/// The lazy-loaded fluent bundles, one per shipped language.
struct Bundles {
    en: FluentBundle<FluentResource>,
    vi: FluentBundle<FluentResource>,
}

impl Bundles {
    fn build() -> Self {
        Self {
            en: build_bundle(EN_FTL, UiLanguage::English.langid()),
            vi: build_bundle(VI_FTL, UiLanguage::Vietnamese.langid()),
        }
    }

    fn for_language(&self, language: UiLanguage) -> &FluentBundle<FluentResource> {
        match language {
            UiLanguage::English => &self.en,
            UiLanguage::Vietnamese => &self.vi,
        }
    }
}

fn build_bundle(ftl: &'static str, langid: LanguageIdentifier) -> FluentBundle<FluentResource> {
    let resource =
        FluentResource::try_new(ftl.to_string()).expect("shipped FTL source must be valid");
    let mut bundle = FluentBundle::new(vec![langid]);
    bundle
        .add_resource(resource)
        .expect("shipped FTL source must parse");
    bundle
}

/// Global i18n state: the current UI language plus its lazily built bundles.
pub struct I18n {
    language: UiLanguage,
    bundles: OnceCell<Bundles>,
}

impl Global for I18n {}

impl I18n {
    pub fn new(language: UiLanguage) -> Self {
        Self {
            language,
            bundles: OnceCell::new(),
        }
    }

    pub fn language(&self) -> UiLanguage {
        self.language
    }

    pub fn set_language(&mut self, language: UiLanguage) {
        self.language = language;
    }

    /// Switch between English and Vietnamese.
    pub fn toggle_language(&mut self) {
        self.set_language(self.language.toggled());
    }

    /// Look up `msg_id` in the current language, falling back to English.
    pub fn tr(&self, msg_id: &str) -> String {
        let bundles = self.bundles.get_or_init(Bundles::build);
        format_message(bundles.for_language(self.language), msg_id)
            .or_else(|| format_message(&bundles.en, msg_id))
            .unwrap_or_else(|| msg_id.to_string())
    }
}

fn format_message(bundle: &FluentBundle<FluentResource>, msg_id: &str) -> Option<String> {
    let message = bundle.get_message(msg_id)?;
    let pattern = message.value()?;
    let mut errors = Vec::new();
    let formatted = bundle.format_pattern(pattern, None, &mut errors);
    Some(formatted.trim().to_string())
}
