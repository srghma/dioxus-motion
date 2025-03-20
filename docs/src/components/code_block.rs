use dioxus::prelude::*;

/// Enhanced syntax highlighting for Rust with special focus on Dioxus-specific patterns
fn highlight_rust_syntax(code: &str) -> String {
    // Create a more robust token-based approach rather than simple replacement
    let mut result = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut token_start = 0;
    let chars: Vec<char> = code.chars().collect();

    for i in 0..chars.len() {
        // Handle comments first
        if !in_string && i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            // Add any accumulated token before the comment
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_token(token, false));
            }

            // Start the comment span
            result.push_str("<span class='text-gray-500'>");
            token_start = i;
            in_comment = true;
            continue;
        }

        // If we're in a comment and hit a newline, close the comment span
        if in_comment && chars[i] == '\n' {
            result.push_str(&code[token_start..=i]);
            result.push_str("</span>");
            token_start = i + 1;
            in_comment = false;
            continue;
        }

        // If we're in a comment, continue to next character
        if in_comment {
            continue;
        }

        // Handle string literals
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            if !in_string {
                // Start of string
                if token_start < i {
                    let token = &code[token_start..i];
                    result.push_str(&highlight_token(token, false));
                }
                result.push_str("<span class='text-green-500'>\"");
                token_start = i + 1;
                in_string = true;
            } else {
                // End of string
                result.push_str(&code[token_start..i]);
                result.push_str("\"</span>");
                token_start = i + 1;
                in_string = false;
            }
            continue;
        }

        // If we're in a string, continue to next character
        if in_string {
            continue;
        }

        // Handle whitespace and separators
        if chars[i].is_whitespace()
            || chars[i] == '{'
            || chars[i] == '}'
            || chars[i] == '('
            || chars[i] == ')'
            || chars[i] == ':'
            || chars[i] == ','
        {
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_token(token, false));
            }

            // Add the separator character as-is
            result.push(chars[i]);
            token_start = i + 1;
        }
    }

    // Add any remaining part
    if token_start < chars.len() {
        let token = &code[token_start..];
        if in_string {
            result.push_str(token);
        } else if in_comment {
            result.push_str(token);
            result.push_str("</span>");
        } else {
            result.push_str(&highlight_token(token, false));
        }
    }

    result
}

/// Helper function to highlight individual tokens
fn highlight_token(token: &str, in_string: bool) -> String {
    if in_string {
        return token.to_string();
    }

    // Clean the token of any color codes that might be present
    let clean_token = token.replace(
        |c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '#' && c != ':',
        "",
    );

    if clean_token.is_empty() {
        return token.to_string();
    }

    // Dioxus-specific attributes
    if clean_token == "#[component]" {
        return "<span class='text-purple-500'>#[component]</span>".to_string();
    }

    // Check for Rust keywords
    let keywords = [
        "fn", "let", "mut", "pub", "use", "struct", "enum", "trait", "impl", "const", "static",
        "async", "await", "for", "while", "loop", "if", "else", "match", "in", "return", "where",
        "type", "dyn",
    ];

    if keywords.contains(&clean_token.as_str()) {
        return format!("<span class='text-blue-500'>{}</span>", token);
    }

    // Dioxus components (capitalized identifiers)
    if !clean_token.is_empty()
        && clean_token.chars().next().unwrap().is_uppercase()
        && !clean_token.starts_with("Route::")
    {
        return format!("<span class='text-orange-400'>{}</span>", token);
    }

    // Handle RSX macro
    if clean_token == "rsx!" {
        return format!("<span class='text-yellow-500'>{}</span>", token);
    }

    // Route types
    if clean_token.starts_with("Route::") {
        let parts: Vec<&str> = clean_token.split("::").collect();
        if parts.len() >= 2 {
            return format!("<span class='text-green-300'>Route::</span><span class='text-orange-400'>{}</span>", 
                         parts[1..].join("::"));
        }
    }

    // Element properties (followed by colon)
    if token.ends_with(':') {
        return format!("<span class='text-blue-300'>{}</span>", token);
    }

    // Numbers
    if clean_token.chars().all(|c| c.is_ascii_digit() || c == '.')
        && clean_token.chars().any(|c| c.is_ascii_digit())
    {
        return format!("<span class='text-orange-400'>{}</span>", token);
    }

    token.to_string()
}

/// Syntax highlighting for TOML files
fn highlight_toml_syntax(code: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut token_start = 0;
    let chars: Vec<char> = code.chars().collect();

    for i in 0..chars.len() {
        // Handle comments first
        if !in_string && i + 1 < chars.len() && chars[i] == '#' {
            // Add any accumulated token before the comment
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_toml_token(token, false));
            }

            // Start the comment span
            result.push_str("<span class='text-gray-500'>");
            token_start = i;
            in_comment = true;
            continue;
        }

        // If we're in a comment and hit a newline, close the comment span
        if in_comment && chars[i] == '\n' {
            result.push_str(&code[token_start..=i]);
            result.push_str("</span>");
            token_start = i + 1;
            in_comment = false;
            continue;
        }

        // If we're in a comment, continue to next character
        if in_comment {
            continue;
        }

        // Handle string literals
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            if !in_string {
                // Start of string
                if token_start < i {
                    let token = &code[token_start..i];
                    result.push_str(&highlight_toml_token(token, false));
                }
                result.push_str("<span class='text-green-500'>\"");
                token_start = i + 1;
                in_string = true;
            } else {
                // End of string
                result.push_str(&code[token_start..i]);
                result.push_str("\"</span>");
                token_start = i + 1;
                in_string = false;
            }
            continue;
        }

        // If we're in a string, continue to next character
        if in_string {
            continue;
        }

        // Handle whitespace and separators
        if chars[i].is_whitespace()
            || chars[i] == '{'
            || chars[i] == '}'
            || chars[i] == '['
            || chars[i] == ']'
            || chars[i] == '='
            || chars[i] == ','
        {
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_toml_token(token, false));
            }

            // Add the separator character with special coloring for brackets
            if chars[i] == '[' || chars[i] == ']' {
                result.push_str(&format!("<span class='text-blue-400'>{}</span>", chars[i]));
            } else {
                result.push(chars[i]);
            }
            token_start = i + 1;
        }
    }

    // Add any remaining part
    if token_start < chars.len() {
        let token = &code[token_start..];
        if in_string {
            result.push_str(token);
        } else if in_comment {
            result.push_str(token);
            result.push_str("</span>");
        } else {
            result.push_str(&highlight_toml_token(token, false));
        }
    }

    result
}

/// Helper function to highlight individual TOML tokens
fn highlight_toml_token(token: &str, in_string: bool) -> String {
    if in_string {
        return token.to_string();
    }

    // Clean the token
    let clean_token = token.trim();

    if clean_token.is_empty() {
        return token.to_string();
    }

    // Handle section headers
    if clean_token.starts_with('[') && clean_token.ends_with(']') {
        return format!("<span class='text-blue-400'>{}</span>", token);
    }

    // Handle key-value pairs
    if token.contains('=') {
        let parts: Vec<&str> = token.split('=').collect();
        if parts.len() >= 2 {
            let key = parts[0].trim();
            let value = parts[1..].join("=").trim().to_string();
            return format!(
                "<span class='text-purple-400'>{}</span>={}",
                key,
                highlight_toml_value(&value)
            );
        }
    }

    // Handle keys
    if token.ends_with('=') {
        return format!("<span class='text-purple-400'>{}</span>", token);
    }

    // Handle version numbers and other literals
    if clean_token
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '"')
    {
        return format!("<span class='text-orange-400'>{}</span>", token);
    }

    token.to_string()
}

/// Helper function to highlight TOML values
fn highlight_toml_value(value: &str) -> String {
    // Handle boolean values
    if value == "true" || value == "false" {
        return format!("<span class='text-orange-400'>{}</span>", value);
    }

    // Handle numbers
    if value
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
    {
        return format!("<span class='text-orange-400'>{}</span>", value);
    }

    // Handle quoted strings
    if value.starts_with('"') && value.ends_with('"') {
        return format!("<span class='text-green-500'>{}</span>", value);
    }

    value.to_string()
}

#[component]
pub fn CodeBlock(code: String, language: String) -> Element {
    let highlighted = match language.to_lowercase().as_str() {
        "rust" => highlight_rust_syntax(&code),
        "toml" => highlight_toml_syntax(&code),
        _ => code.clone(),
    };

    rsx! {
        pre {
            class: format!(
                "language-{} overflow-x-auto rounded-lg bg-dark-300/50 p-4 font-mono",
                language,
            ),
            dangerous_inner_html: "{highlighted}",
        }
    }
}
