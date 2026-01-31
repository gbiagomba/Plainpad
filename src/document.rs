use ropey::Rope;
use std::{fs, io, path::PathBuf};

pub struct Document {
    text: String,
    rope: Rope,
    path: Option<PathBuf>,
    untitled_name: String,
    dirty: bool,
}

impl Document {
    pub fn new_untitled(index: usize) -> Self {
        let name = format!("Untitled {}", index);
        Self {
            text: String::new(),
            rope: Rope::new(),
            path: None,
            untitled_name: name,
            dirty: false,
        }
    }

    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let text = fs::read_to_string(&path)?;
        let rope = Rope::from_str(&text);
        let untitled_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Untitled")
            .to_string();

        Ok(Self {
            text,
            rope,
            path: Some(path),
            untitled_name,
            dirty: false,
        })
    }

    pub fn title(&self) -> String {
        if let Some(path) = &self.path {
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&self.untitled_name)
                .to_string()
        } else {
            self.untitled_name.clone()
        }
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn sync_rope(&mut self) {
        self.rope = Rope::from_str(&self.text);
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn save_to(&mut self, path: PathBuf) -> io::Result<()> {
        fs::write(&path, &self.text)?;
        self.path = Some(path);
        self.dirty = false;
        Ok(())
    }
}
