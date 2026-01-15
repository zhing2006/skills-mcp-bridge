use rmcp::model::{
    CallToolResult, Content, PromptMessage, PromptMessageContent, PromptMessageRole, RawContent,
    ResourceContents,
};

pub(crate) fn call_tool_result_to_text(result: &CallToolResult) -> String {
    if !result.content.is_empty() {
        return result
            .content
            .iter()
            .map(content_to_text)
            .collect::<Vec<_>>()
            .join("\n");
    }

    if let Some(structured) = &result.structured_content {
        return structured.to_string();
    }

    String::new()
}

fn content_to_text(content: &Content) -> String {
    match &content.raw {
        RawContent::Text(text) => text.text.clone(),
        RawContent::Image(image) => format!(
            "[image mime_type={} bytes={}]",
            image.mime_type,
            image.data.len()
        ),
        RawContent::Audio(audio) => format!(
            "[audio mime_type={} bytes={}]",
            audio.mime_type,
            audio.data.len()
        ),
        RawContent::Resource(resource) => resource_contents_to_text(&resource.resource),
        RawContent::ResourceLink(link) => format!("[resource link uri={}]", link.uri),
    }
}

pub(crate) fn resource_contents_to_text(contents: &ResourceContents) -> String {
    match contents {
        ResourceContents::TextResourceContents { text, .. } => text.clone(),
        ResourceContents::BlobResourceContents { blob, .. } => blob.clone(),
    }
}

pub(crate) fn prompt_messages_to_text(messages: &[PromptMessage]) -> String {
    messages
        .iter()
        .map(|message| {
            let role = match message.role {
                PromptMessageRole::User => "user",
                PromptMessageRole::Assistant => "assistant",
            };
            let content = prompt_content_to_text(&message.content);
            format!("{role}: {content}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn prompt_content_to_text(content: &PromptMessageContent) -> String {
    match content {
        PromptMessageContent::Text { text } => text.clone(),
        PromptMessageContent::Image { image } => format!(
            "[image mime_type={} bytes={}]",
            image.mime_type,
            image.data.len()
        ),
        PromptMessageContent::Resource { resource } => {
            resource_contents_to_text(&resource.resource)
        }
        PromptMessageContent::ResourceLink { link } => format!("[resource link uri={}]", link.uri),
    }
}
