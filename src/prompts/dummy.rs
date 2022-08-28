use std::io;
use crate::{paging::Paging, term::Term, theme::{SimpleTheme, TermThemeRenderer}};

fn test(term: Term) -> io::Result<()> {
  let mut paging = Paging::new(Term::clone(&term), 0, Some(42));
  let mut render = TermThemeRenderer::new(Term::clone(&term), &SimpleTheme);

  term.hide_cursor()?;

  paging.render_prompt(|paging_info| render.select_prompt("My prompt string", paging_info))?;

  Ok(())
}
