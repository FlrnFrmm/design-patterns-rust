use anyhow::{Result, Context};
use cursive::{views::EditView, Cursive};

use super::Command;
use crate::AppContext;

#[derive(Default)]
pub struct CopyCommand;

impl Command for CopyCommand {
    fn execute(&mut self, app: &mut Cursive) -> Result<()> {
        let editor = app.find_name::<EditView>("Editor")
            .context("Couldn't access editor view")?;
        let mut context = app.take_user_data::<AppContext>()
            .context("Couldn't access user data")?;

        context.clipboard = editor.get_content().to_string();

        app.set_user_data(context);

        Ok(())
    }

    fn undo(&mut self, _: &mut Cursive) {}
}
