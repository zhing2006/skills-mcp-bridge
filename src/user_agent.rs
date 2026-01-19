use std::fmt;
use std::str::FromStr;

/// Predefined User-Agent presets.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UserAgentPreset {
    /// Google Chrome browser (default)
    #[default]
    Chrome,
    /// Microsoft Edge browser
    Edge,
    /// Mozilla Firefox browser
    Firefox,
    /// Apple Safari browser
    Safari,
    /// Microsoft Internet Explorer (legacy)
    Ie,
    /// Anthropic Claude Code CLI
    ClaudeCode,
    /// OpenAI Codex CLI
    Codex,
    /// Google Gemini CLI
    GeminiCli,
    /// OpenCode AI coding assistant
    OpenCode,
    /// Cursor AI editor
    Cursor,
    /// Custom User-Agent string
    Custom(String),
}

impl UserAgentPreset {
    /// Returns the User-Agent header value for this preset.
    pub fn user_agent(&self) -> &str {
        match self {
            // Browser User-Agents (latest versions as of 2025)
            UserAgentPreset::Chrome => {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
            }
            UserAgentPreset::Edge => {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"
            }
            UserAgentPreset::Firefox => {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:134.0) Gecko/20100101 Firefox/134.0"
            }
            UserAgentPreset::Safari => {
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_7_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2.1 Safari/605.1.15"
            }
            UserAgentPreset::Ie => {
                "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko"
            }

            // AI Coding Tools User-Agents
            UserAgentPreset::ClaudeCode => "claude-code/1.0.33",
            UserAgentPreset::Codex => "codex-cli/1.0.0",
            UserAgentPreset::GeminiCli => "gemini-cli/0.1.0 google-genai-sdk/0.5.0",
            UserAgentPreset::OpenCode => "opencode/0.1.0",
            UserAgentPreset::Cursor => "cursor/0.50.0",

            UserAgentPreset::Custom(ua) => ua.as_str(),
        }
    }

    /// Returns the MCP client name for this preset.
    pub fn client_name(&self) -> &str {
        match self {
            UserAgentPreset::Chrome => "Chrome",
            UserAgentPreset::Edge => "Microsoft Edge",
            UserAgentPreset::Firefox => "Firefox",
            UserAgentPreset::Safari => "Safari",
            UserAgentPreset::Ie => "Internet Explorer",
            UserAgentPreset::ClaudeCode => "claude-code",
            UserAgentPreset::Codex => "codex-cli",
            UserAgentPreset::GeminiCli => "gemini-cli",
            UserAgentPreset::OpenCode => "opencode",
            UserAgentPreset::Cursor => "cursor",
            UserAgentPreset::Custom(_) => "custom",
        }
    }

    /// Returns the MCP client version for this preset.
    pub fn client_version(&self) -> &str {
        match self {
            UserAgentPreset::Chrome => "131.0.0.0",
            UserAgentPreset::Edge => "131.0.0.0",
            UserAgentPreset::Firefox => "134.0",
            UserAgentPreset::Safari => "18.2.1",
            UserAgentPreset::Ie => "11.0",
            UserAgentPreset::ClaudeCode => "1.0.33",
            UserAgentPreset::Codex => "1.0.0",
            UserAgentPreset::GeminiCli => "0.1.0",
            UserAgentPreset::OpenCode => "0.1.0",
            UserAgentPreset::Cursor => "0.50.0",
            UserAgentPreset::Custom(_) => "1.0.0",
        }
    }
}

impl FromStr for UserAgentPreset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_ascii_lowercase();
        match lower.as_str() {
            "chrome" => Ok(UserAgentPreset::Chrome),
            "edge" => Ok(UserAgentPreset::Edge),
            "firefox" | "ff" => Ok(UserAgentPreset::Firefox),
            "safari" => Ok(UserAgentPreset::Safari),
            "ie" | "internet-explorer" => Ok(UserAgentPreset::Ie),
            "claude-code" | "claudecode" | "claude" => Ok(UserAgentPreset::ClaudeCode),
            "codex" | "codex-cli" => Ok(UserAgentPreset::Codex),
            "gemini-cli" | "gemini" | "geminicli" => Ok(UserAgentPreset::GeminiCli),
            "opencode" | "open-code" => Ok(UserAgentPreset::OpenCode),
            "cursor" => Ok(UserAgentPreset::Cursor),
            _ => {
                // Treat any other string as a custom User-Agent
                if s.is_empty() {
                    return Err("User-Agent cannot be empty".to_string());
                }
                Ok(UserAgentPreset::Custom(s.to_string()))
            }
        }
    }
}

impl fmt::Display for UserAgentPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAgentPreset::Chrome => write!(f, "chrome"),
            UserAgentPreset::Edge => write!(f, "edge"),
            UserAgentPreset::Firefox => write!(f, "firefox"),
            UserAgentPreset::Safari => write!(f, "safari"),
            UserAgentPreset::Ie => write!(f, "ie"),
            UserAgentPreset::ClaudeCode => write!(f, "claude-code"),
            UserAgentPreset::Codex => write!(f, "codex"),
            UserAgentPreset::GeminiCli => write!(f, "gemini-cli"),
            UserAgentPreset::OpenCode => write!(f, "opencode"),
            UserAgentPreset::Cursor => write!(f, "cursor"),
            UserAgentPreset::Custom(ua) => write!(f, "{}", ua),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_preset() {
        assert_eq!(
            "chrome".parse::<UserAgentPreset>().unwrap(),
            UserAgentPreset::Chrome
        );
        assert_eq!(
            "claude-code".parse::<UserAgentPreset>().unwrap(),
            UserAgentPreset::ClaudeCode
        );
        assert_eq!(
            "CHROME".parse::<UserAgentPreset>().unwrap(),
            UserAgentPreset::Chrome
        );
    }

    #[test]
    fn test_parse_custom() {
        let result = "MyApp/1.0".parse::<UserAgentPreset>().unwrap();
        assert_eq!(result.user_agent(), "MyApp/1.0");
    }

    #[test]
    fn test_default() {
        assert_eq!(UserAgentPreset::default(), UserAgentPreset::Chrome);
    }

    #[test]
    fn test_user_agent() {
        assert!(UserAgentPreset::Chrome.user_agent().contains("Chrome"));
        assert!(
            UserAgentPreset::ClaudeCode
                .user_agent()
                .contains("claude-code")
        );
    }
}
