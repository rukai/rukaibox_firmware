use arrayvec::ArrayVec;
use kdl::{KdlDocument, KdlEntry, KdlError, KdlNode};
use kdl_config_derive::KdlConfig;
use miette::{Diagnostic, NamedSource, SourceOffset, SourceSpan};
use thiserror::Error;

pub fn load() -> miette::Result<ConfigParsed> {
    let input = NamedSource::new("config.kdl", load_text()?);
    // TODO: upstream a way to tell KDL parser what the filename is.
    let kdl: KdlDocument = input.inner().parse()?;
    let mut diag = vec![];
    let [profiles_node] = get_children_of_document(input.clone(), &kdl, ["profiles"], &mut diag);

    let mut profiles = ArrayVec::new();

    if let Some(profiles_node) = profiles_node {
        for profile_node in profiles_node
            .children()
            .map(|x| x.nodes())
            .unwrap_or_default()
        {
            profiles.push(KdlConfig::parse_as_node(
                input.clone(),
                profile_node,
                &mut diag,
            ));
        }
    }
    if diag.is_empty() {
        Ok(ConfigParsed {
            profiles: Parsed {
                value: profiles,
                full_span: profiles_node.unwrap().span(),
                name_span: profiles_node.unwrap().span(),
                valid: true,
            },
        })
    } else {
        Err(ParseError {
            input,
            diagnostics: diag,
        }
        .into())
    }
}

fn get_children<'a, const N: usize>(
    input: NamedSource<String>,
    node: &'a KdlNode,
    names: [&str; N],
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> [Option<&'a KdlNode>; N] {
    match node.children() {
        Some(children) => get_children_of_document(input, children, names, diagnostics),
        None => {
            diagnostics.push(ParseDiagnostic {
                input: input.clone(),
                span: node.span(),
                message: Some(format!(
                    "Node has no children but expected children with names {names:?}"
                )),
                label: None,
                help: None,
                severity: miette::Severity::Error,
            });
            [None; N]
        }
    }
}

fn get_children_of_document<'a, const N: usize>(
    input: NamedSource<String>,
    children: &'a KdlDocument,
    names: [&str; N],
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> [Option<&'a KdlNode>; N] {
    let mut result_children = vec![];
    let mut missing_fields = vec![];
    for name in names {
        if let Some(child) = children.get(name) {
            result_children.push(Some(child))
        } else {
            result_children.push(None);
            diagnostics.push(ParseDiagnostic {
                input: input.clone(),
                span: children.span(),
                message: Some(format!("Child {name} is missing from this node")),
                label: None,
                help: None,
                severity: miette::Severity::Error,
            });
            missing_fields.push(name);
        }
    }

    for child in children.nodes() {
        if !names.contains(&child.name().value()) {
            diagnostics.push(ParseDiagnostic {
                input: input.clone(),
                span: child.span(),
                message: Some("Unknown node name".to_owned()),
                label: None,
                help: Some(if missing_fields.is_empty() {
                    "This node already has all the children it needs. Consider removing this section.".to_owned()
                } else {
                    format!("Consider one of these {names:?} instead?")
                }),
                severity: miette::Severity::Error,
            });
        }
    }
    result_children.try_into().unwrap()
}

fn load_text() -> Result<String, KdlError> {
    Ok(include_str!("../../config.kdl").to_owned())
}

#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, Error)]
#[error("Failed to parse configuration")]
pub struct ParseError {
    /// Original input that this failure came from.
    #[source_code]
    pub input: NamedSource<String>,

    /// Sub-diagnostics for this failure.
    #[related]
    pub diagnostics: Vec<ParseDiagnostic>,
}

/// An individual diagnostic message for a KDL parsing issue.
///
/// While generally signifying errors, they can also be treated as warnings.
#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, Error)]
#[error("{}", message.clone().unwrap_or_else(|| "Unexpected error".into()))]
pub struct ParseDiagnostic {
    /// Shared source for the diagnostic.
    #[source_code]
    pub input: NamedSource<String>,

    /// Offset in chars of the error.
    #[label("{}", label.clone().unwrap_or_else(|| "here".into()))]
    pub span: SourceSpan,

    /// Message for the error itself.
    pub message: Option<String>,

    /// Label text for this span. Defaults to `"here"`.
    pub label: Option<String>,

    /// Suggestion for fixing the parser error.
    #[help]
    pub help: Option<String>,

    /// Severity level for the Diagnostic.
    #[diagnostic(severity)]
    pub severity: miette::Severity,
}

/// manually implement for now, derive this later
pub trait KdlConfig {
    fn parse_as_node(
        source: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized;
    fn parse_as_entry(
        _source: NamedSource<String>,
        entry: &KdlEntry,
        _diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized,
    {
        let type_name = std::any::type_name::<Self>();
        let entry = entry.to_string();
        unimplemented!(
            "Tried to parse entry {entry:?} as {type_name}. However {type_name} does not have an implementation for parse_as_entry."
        )
    }
}

impl<T: KdlConfig, const CAP: usize> KdlConfig for ArrayVec<Parsed<T>, CAP> {
    fn parse_as_node(
        source: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized,
    {
        let mut array = ArrayVec::new();
        // TODO: provide "children as a list" as an alternative parsing style
        for entry in node.entries() {
            array.push(KdlConfig::parse_as_entry(
                source.clone(),
                entry,
                diagnostics,
            ))
        }
        Parsed {
            value: array,
            full_span: node.span(),
            name_span: node.span(),
            valid: true,
        }
    }
}

pub struct Parsed<T> {
    /// The actual parsed value
    pub value: T,
    /// The span of the entire KDL node
    pub full_span: SourceSpan,
    /// The span of the KDL nodes identifier
    pub name_span: SourceSpan,
    /// When a field cannot be parsed, this field is set to `false` and `value` is set to `Default::default`
    pub valid: bool,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Parsed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parsed")
            .field("value", &self.value)
            .field("valid", &self.valid)
            .finish()
    }
}

impl<T: Default> Default for Parsed<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            full_span: SourceSpan::new(SourceOffset::from_location("", 0, 0), 0),
            name_span: SourceSpan::new(SourceOffset::from_location("", 0, 0), 0),
            valid: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ConfigParsed {
    pub profiles: Parsed<ArrayVec<Parsed<ProfileParsed>, 10>>,
}

#[derive(KdlConfig, Default, Debug)]
pub struct ProfileParsed {
    //pub default: Parsed<bool>,
    pub activation_combination: Parsed<ArrayVec<Parsed<PhysicalButtonParsed>, 10>>,
    pub logic: Parsed<BaseLogicParsed>,
    pub socd: Parsed<SocdTypeParsed>,
    pub left_hand: Parsed<LeftHandMapParsed>,
    pub right_hand: Parsed<RightHandMapParsed>,
}

#[derive(KdlConfig, Default, Debug)]
pub struct LeftHandMapParsed {
    pub pinky: Parsed<LogicalButtonParsed>,
    pub ring: Parsed<LogicalButtonParsed>,
    pub middle: Parsed<LogicalButtonParsed>,
    pub index: Parsed<LogicalButtonParsed>,

    pub middle_2: Parsed<LogicalButtonParsed>,

    pub thumb_left: Parsed<LogicalButtonParsed>,
    pub thumb_right: Parsed<LogicalButtonParsed>,
}

#[derive(KdlConfig, Default, Debug)]
pub struct RightHandMapParsed {
    pub index: Parsed<LogicalButtonParsed>,
    pub middle: Parsed<LogicalButtonParsed>,
    pub ring: Parsed<LogicalButtonParsed>,
    pub pinky: Parsed<LogicalButtonParsed>,

    pub index_2: Parsed<LogicalButtonParsed>,
    pub middle_2: Parsed<LogicalButtonParsed>,
    pub ring_2: Parsed<LogicalButtonParsed>,
    pub pinky_2: Parsed<LogicalButtonParsed>,

    pub thumb_left: Parsed<LogicalButtonParsed>,
    pub thumb_right: Parsed<LogicalButtonParsed>,
    pub thumb_up: Parsed<LogicalButtonParsed>,
    pub thumb_down: Parsed<LogicalButtonParsed>,
    pub thumb_middle: Parsed<LogicalButtonParsed>,
}

#[derive(KdlConfig, Default, Debug)]
pub enum SocdTypeParsed {
    #[default]
    SecondInputPriority,
    Neutral,
}

#[derive(KdlConfig, Default, Debug)]
pub enum BaseLogicParsed {
    #[default]
    ProjectPlus,
    Rivals2,
}

#[derive(KdlConfig, Default, Debug)]
pub enum PhysicalButtonParsed {
    #[default]
    Start,
    LeftHandPinky,
    LeftHandRing,
    LeftHandMiddle,
    LeftHandIndex,

    LeftHandMiddle2,

    LeftHandThumbLeft,
    LeftHandThumbRight,

    RightHandIndex,
    RightHandMiddle,
    RightHandRing,
    RightHandPink,

    RightHandIndex2,
    RightHandMiddle2,
    RightHandRing2,
    RightHandPink2,

    RightHandThumbLeft,
    RightHandThumbRight,
    RightHandThumbUp,
    RightHandThumbDown,
    RightHandThumbMiddle,
}

#[derive(KdlConfig, Default, Debug)]
pub enum LogicalButtonParsed {
    #[default]
    LAnalog,
    RAnalog,
    LDigital,
    RDigital,
    StickUp,
    StickDown,
    StickLeft,
    StickRight,
    CstickUp,
    CstickDown,
    CstickLeft,
    CstickRight,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    ModX,
    ModY,
    A,
    B,
    X,
    Y,
    Z,
}
