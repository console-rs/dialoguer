use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    io::{Read, Write},
    path::Path,
    process,
};

use crate::Result;

/// Launches the default editor to edit a string.
///
/// ## Example
///
/// ```rust,no_run
/// use dialoguer::Editor;
///
/// if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap() {
///     println!("Your message:");
///     println!("{}", rv);
/// } else {
///     println!("Abort!");
/// }
/// ```
pub struct Editor {
    editor: OsString,
    extension: String,
    require_save: bool,
    trim_newlines: bool,
}

fn get_default_editor() -> OsString {
    if let Some(prog) = env::var_os("VISUAL") {
        return prog;
    }
    if let Some(prog) = env::var_os("EDITOR") {
        return prog;
    }
    if cfg!(windows) {
        "notepad.exe".into()
    } else {
        "vi".into()
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    /// Creates a new editor.
    pub fn new() -> Self {
        Self {
            editor: get_default_editor(),
            extension: ".txt".into(),
            require_save: true,
            trim_newlines: true,
        }
    }

    /// Sets a specific editor executable.
    pub fn executable<S: AsRef<OsStr>>(&mut self, val: S) -> &mut Self {
        self.editor = val.as_ref().into();
        self
    }

    /// Sets a specific extension
    pub fn extension(&mut self, val: &str) -> &mut Self {
        self.extension = val.into();
        self
    }

    /// Enables or disables the save requirement.
    pub fn require_save(&mut self, val: bool) -> &mut Self {
        self.require_save = val;
        self
    }

    /// Enables or disables trailing newline stripping.
    ///
    /// This is on by default.
    pub fn trim_newlines(&mut self, val: bool) -> &mut Self {
        self.trim_newlines = val;
        self
    }

    /// Spawns the configured editor pointed at `path` and waits for it to exit.
    fn run_editor(&self, path: &Path) -> Result<process::ExitStatus> {
        let s: String = self.editor.clone().into_string().unwrap();
        let (cmd, args) = match shell_words::split(&s) {
            Ok(mut parts) => {
                let cmd = parts.remove(0);
                (cmd, parts)
            }
            Err(_) => (s, vec![]),
        };

        Ok(process::Command::new(cmd)
            .args(args)
            .arg(path)
            .spawn()?
            .wait()?)
    }

    /// Launches the editor to edit a string.
    ///
    /// Returns `None` if the file was not saved or otherwise the
    /// entered text.
    pub fn edit(&self, s: &str) -> Result<Option<String>> {
        let mut f = tempfile::Builder::new()
            .prefix("edit-")
            .suffix(&self.extension)
            .rand_bytes(12)
            .tempfile()?;
        f.write_all(s.as_bytes())?;
        f.flush()?;
        let ts = fs::metadata(f.path())?.modified()?;

        let rv = self.run_editor(f.path())?;

        if rv.success() && self.require_save && ts >= fs::metadata(f.path())?.modified()? {
            return Ok(None);
        }

        let mut new_f = fs::File::open(f.path())?;
        let mut rv = String::new();
        new_f.read_to_string(&mut rv)?;

        if self.trim_newlines {
            let len = rv.trim_end_matches(&['\n', '\r'][..]).len();
            rv.truncate(len);
        }

        Ok(Some(rv))
    }

    /// Launches the editor to edit an existing file in place.
    ///
    /// Unlike [`edit`](Self::edit), the file at `path` is opened directly, so
    /// its contents are never copied through a temporary file and the editor
    /// writes any changes back itself. That avoids duplicating the file in
    /// memory, which is useful for large files.
    ///
    /// Returns `Ok(true)` when the file was saved. If
    /// [`require_save`](Self::require_save) is enabled (the default) and the
    /// editor left the file untouched, `Ok(false)` is returned instead.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::Editor;
    ///
    /// if Editor::new().edit_file("notes.txt").unwrap() {
    ///     println!("Saved!");
    /// } else {
    ///     println!("No changes.");
    /// }
    /// ```
    pub fn edit_file<P: AsRef<Path>>(&self, path: P) -> Result<bool> {
        let path = path.as_ref();
        let ts = fs::metadata(path)?.modified()?;

        let rv = self.run_editor(path)?;

        if rv.success() && self.require_save && ts >= fs::metadata(path)?.modified()? {
            return Ok(false);
        }

        Ok(true)
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::Editor;
    use std::io::Write;

    #[test]
    fn edit_file_reports_saved_when_modified() {
        let mut f = tempfile::Builder::new().tempfile().unwrap();
        write!(f, "before").unwrap();
        f.flush().unwrap();

        // Fake editor that appends to the file it is handed as the last argument.
        let saved = Editor::new()
            .executable("sh -c 'printf changed >> \"$1\"' sh")
            .edit_file(f.path())
            .unwrap();

        assert!(saved);
    }

    #[test]
    fn edit_file_reports_not_saved_when_untouched() {
        let f = tempfile::Builder::new().tempfile().unwrap();

        // `true` succeeds without touching the file.
        let saved = Editor::new()
            .executable("true")
            .edit_file(f.path())
            .unwrap();

        assert!(!saved);
    }
}
