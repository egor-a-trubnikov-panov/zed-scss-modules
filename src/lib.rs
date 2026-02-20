use zed_extension_api::{self as zed, serde_json};

struct ScssModules;

impl zed::Extension for ScssModules {
    /// Обязательный конструктор (API 0.6)
    fn new() -> Self {
        Self
    }

    /// Запуск cssmodules‑language‑server
    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if id.as_ref() != "scss-modules-ls" {
            return Err(format!("Unhandled LS id: {id}"));
        }

        let exe = _worktree
            .which("cssmodules-language-server")
            .ok_or_else(|| {
                "`cssmodules-language-server` not found. Run `npm i -g cssmodules-language-server`"
                    .to_string()
            })?;

        Ok(zed::Command {
            command: exe,
            args: vec!["--stdio".into()],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        if id.as_ref() != "scss-modules-ls" {
            return Ok(None);
        }

        // cssmodules-language-server uses camelCase=true by default.
        // For selectors like `.EggWrapper` this breaks go-to-definition from
        // `styles.EggWrapper` (it expects `styles.eggWrapper`).
        Ok(Some(serde_json::json!({
            "camelCase": false
        })))
    }
}

zed::register_extension!(ScssModules);
