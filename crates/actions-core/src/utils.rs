use crate::command::CommandProperties;
use crate::AnnotationProperties;

pub fn to_command_value(value: Option<String>) -> String {
    value.unwrap_or_default()
}

pub fn to_command_properties<'a>(
    annotatation_properties: AnnotationProperties,
) -> CommandProperties {
    CommandProperties::from([
        (
            "title".into(),
            annotatation_properties.title.unwrap_or_default().into(),
        ),
        (
            "file".into(),
            annotatation_properties.file.unwrap_or_default().into(),
        ),
        (
            "line".into(),
            format!("{}", annotatation_properties.start_line.unwrap_or_default()),
        ),
        (
            "endLine".into(),
            format!("{}", annotatation_properties.end_line.unwrap_or_default()),
        ),
        (
            "col".into(),
            format!(
                "{}",
                annotatation_properties.start_column.unwrap_or_default()
            ),
        ),
        (
            "endColumn".into(),
            format!("{}", annotatation_properties.end_column.unwrap_or_default()),
        ),
    ])
}
