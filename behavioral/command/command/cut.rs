use anyhow::{Result, Context};
use cursive::{views::EditView, Cursive};

use super::Command;
use crate::AppContext;

#[derive(Default)]
pub struct CutCommand {
    backup: String,
}

impl Command for CutCommand {
    fn execute(&mut self, app: &mut Cursive) -> Result<()>  {
        let mut editor = app.find_name::<EditView>("Editor")
            .context("Couldn't access editor view")?;

        app.with_user_data(|context: &mut AppContext| {
            self.backup = editor.get_content().to_string();
            context.clipboard = self.backup.clone();
            editor.set_content("".to_string());
        });

        Ok(())
    }

    fn undo(&mut self, app: &mut Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        editor.set_content(&self.backup);
    }
}
