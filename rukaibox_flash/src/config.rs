use arrayvec::ArrayVec;
use kdl::{KdlDocument, KdlError, KdlNode};
use miette::{Diagnostic, NamedSource, SourceSpan};
use rukaibox_config::{BaseLogic, Config, LeftHandMap, Profile};
use thiserror::Error;

pub fn load() -> miette::Result<Config> {
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
            if let [
                Some(_default),
                Some(_activation_combination),
                Some(logic),
                Some(_socd),
                Some(left_hand),
                Some(_right_hand),
            ] = get_children(
                input.clone(),
                profile_node,
                [
                    "default",
                    "activation_combination",
                    "logic",
                    "socd",
                    "left-hand",
                    "right-hand",
                ],
                &mut diag,
            ) {
                let logic = parse_logic(input.clone(), logic, &mut diag);
                let left_hand = parse_left_hand(input.clone(), left_hand, &mut diag);
                if let (Some(logic), Some(left_hand)) = (logic, left_hand) {
                    profiles.push(Profile {
                        default: Default::default(),
                        activation_combination: Default::default(),
                        logic,
                        socd: Default::default(),
                        left_hand,
                        right_hand: Default::default(),
                    });
                }
            }
        }
    }
    if diag.is_empty() {
        Ok(Config { profiles })
    } else {
        Err(ParseError {
            input,
            diagnostics: diag,
        }
        .into())
    }
}

fn parse_logic(
    input: NamedSource<String>,
    node: &KdlNode,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Option<BaseLogic> {
    let entry_len = node.entries().len();
    if node.entries().len() == 1 {
        Some(BaseLogic::ProjectPlus)
    } else {
        let extra_entries: Vec<String> = node
            .entries()
            .iter()
            .skip(1)
            .map(|x| x.value().to_string())
            .collect();
        diagnostics.push(ParseDiagnostic {
            input: input.clone(),
            span: node.span(),
            message: Some(format!(
                "Node should only contain 1 entry but contained {entry_len:?}"
            )),
            label: None,
            help: Some(format!(
                "Consider removing the extra entries {extra_entries:?}",
            )),
            severity: miette::Severity::Error,
        });
        None
    }
}

fn parse_left_hand(
    input: NamedSource<String>,
    node: &KdlNode,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Option<LeftHandMap> {
    match get_children(
        input.clone(),
        node,
        [
            "pinky",
            "ring",
            "middle",
            "index",
            "middle-2",
            "thumb-left",
            "thumb-right",
        ],
        diagnostics,
    ) {
        [
            Some(_pinky),
            Some(_ring),
            Some(_middle),
            Some(_index),
            Some(_middle_2),
            Some(_thumb_left),
            Some(_thumb_right),
        ] => Some(Default::default()),
        _ => None,
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
