use crate::document::Document;
use std::{io, path::PathBuf};

pub struct Editor {
    documents: Vec<Document>,
    active: usize,
    untitled_counter: usize,
}

impl Editor {
    pub fn new() -> Self {
        let mut editor = Self {
            documents: Vec::new(),
            active: 0,
            untitled_counter: 1,
        };
        editor.new_document();
        editor
    }

    pub fn documents(&self) -> &[Document] {
        &self.documents
    }

    pub fn current_mut(&mut self) -> Option<&mut Document> {
        self.documents.get_mut(self.active)
    }

    pub fn current(&self) -> Option<&Document> {
        self.documents.get(self.active)
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.documents.len() {
            self.active = index;
        }
    }

    pub fn new_document(&mut self) {
        let doc = Document::new_untitled(self.untitled_counter);
        self.untitled_counter += 1;
        self.documents.push(doc);
        self.active = self.documents.len().saturating_sub(1);
    }

    pub fn open_document(&mut self, path: PathBuf) -> io::Result<()> {
        let doc = Document::from_path(path)?;
        self.documents.push(doc);
        self.active = self.documents.len().saturating_sub(1);
        Ok(())
    }

    pub fn close_document(&mut self, index: usize) {
        if index >= self.documents.len() {
            return;
        }

        self.documents.remove(index);

        if self.documents.is_empty() {
            self.new_document();
            return;
        }

        if index < self.active {
            self.active = self.active.saturating_sub(1);
        }

        if self.active >= self.documents.len() {
            self.active = self.documents.len().saturating_sub(1);
        }
    }

    pub fn save_document(&mut self, index: usize, path: PathBuf) -> io::Result<()> {
        if let Some(doc) = self.documents.get_mut(index) {
            doc.save_to(path)?;
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        if self.documents.is_empty() {
            return;
        }
        self.active = (self.active + 1) % self.documents.len();
    }

    pub fn previous_tab(&mut self) {
        if self.documents.is_empty() {
            return;
        }
        if self.active == 0 {
            self.active = self.documents.len().saturating_sub(1);
        } else {
            self.active -= 1;
        }
    }
}
