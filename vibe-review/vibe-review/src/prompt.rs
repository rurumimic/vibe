use crate::context::ReviewContext;
use crate::git::DiffFile;

const SYSTEM_PROMPT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/prompts/system.md"));

pub fn build_prompt(context: &ReviewContext) -> String {
    let mut content = String::new();
    content.push_str(SYSTEM_PROMPT);
    content.push_str("\n\n## Review Context\n\n");

    if let Some(style_guide) = &context.style_guide {
        content.push_str("### Style Guide\n\n");
        content.push_str(style_guide);
        content.push_str("\n\n");
    }

    if let Some(readme) = &context.project_readme {
        content.push_str("### Project README\n\n");
        content.push_str(readme);
        content.push_str("\n\n");
    }

    content.push_str("### Diff\n\n");
    for file in &context.diff.files {
        append_diff_file(&mut content, file);
    }

    content
}

fn append_diff_file(content: &mut String, file: &DiffFile) {
    content.push_str("#### ");
    content.push_str(&file.path);
    content.push_str("\n\n");
    content.push_str("```diff\n");
    content.push_str(&file.diff);
    content.push_str("\n```\n\n");
}
