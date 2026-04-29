use crate::Component;

fn escape_script_content(js: &str) -> String {
    js.replace("</script>", r"<\/script>")
        .replace("</Script>", r"<\/Script>")
        .replace("</SCRIPT>", r"<\/SCRIPT>")
        .replace("</ script>", r"<\/ script>")
}

pub struct AzumiScript;

impl Component for AzumiScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<script>{}</script>",
            escape_script_content(crate::AZUMI_JS)
        )
    }
}

pub struct SessionCleanupScript;

impl SessionCleanupScript {
    pub const SCRIPT: &'static str = r#"(function(){var h=window.location.hash||'';var t='session'+'_'+'token'+'=';var r='refresh'+'_'+'token'+'=';var c='code'+'=';if(h.indexOf(t)!==-1||h.indexOf(r)!==-1||h.indexOf(c)!==-1){history.replaceState(null,'',window.location.pathname+window.location.search);}})()"#;
}

impl Component for SessionCleanupScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<script>{}</script>", Self::SCRIPT)
    }
}

pub fn session_cleanup_script() -> SessionCleanupScript {
    SessionCleanupScript
}

pub struct TrustedHtml(pub String);

impl Component for TrustedHtml {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TrustedHtml {
    pub fn new(html: &str) -> Self {
        TrustedHtml(html.to_string())
    }
}
