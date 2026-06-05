use crate::config::KeyBindingsConfig;
use crate::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::text::{Line, Text};
use tui_popup::Popup;

pub struct HelpWidget {
    kb_config: KeyBindingsConfig,
    theme: Theme,
}

impl HelpWidget {
    pub fn new(kb_config: KeyBindingsConfig, theme: Theme) -> Self {
        Self { kb_config, theme }
    }
}

impl Widget for &HelpWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        #[rustfmt::skip]
        let lines = Text::from(vec![
            Line::from(format!("{:012} - Execute full command", self.kb_config.execute_full.first().unwrap().to_string())),
            Line::from(format!("{:012} - Execute until cursor", self.kb_config.execute_until_current.first().unwrap().to_string())),
            Line::from(format!("{:012} - Execute before cursor", self.kb_config.execute_until_prev.first().unwrap().to_string())),
            Line::from(format!("{:012} - Reset input", self.kb_config.reset_input.first().unwrap().to_string())),
            Line::from(""),
            Line::from(format!("{:012} - Save output to file", self.kb_config.save_output.first().unwrap().to_string())),
            Line::from(format!("{:012} - Save command to file", self.kb_config.save_command.first().unwrap().to_string())),
            Line::from(""),
            Line::from(format!("{:012} - Search next", self.kb_config.search_next.first().unwrap().to_string())),
            Line::from(format!("{:012} - Search previous", self.kb_config.search_prev.first().unwrap().to_string())),
            Line::from(format!("{:012} - Toggle regex mode", "alt+x")),
            Line::from(format!("{:012} - Toggle case sensitivity", "alt+c")),
            Line::from(""),
            Line::from(format!("{:012} - Complete forward", self.kb_config.complete.first().unwrap().to_string())),
            Line::from(format!("{:012} - Complete backward", self.kb_config.complete_prev.first().unwrap().to_string())),
            Line::from(""),
            Line::from(format!("{:012} - Go to previous subcommand", self.kb_config.subcommand_prev.first().unwrap().to_string())),
            Line::from(format!("{:012} - Go to next subcommand", self.kb_config.subcommand_next.first().unwrap().to_string())),
            Line::from(format!("{:012} - Copy current subcommand", self.kb_config.subcommand_copy.first().unwrap().to_string())),
            Line::from(format!("{:012} - Cut current subcommand", self.kb_config.subcommand_cut.first().unwrap().to_string())),
            Line::from(format!("{:012} - Paste subcommand after", self.kb_config.subcommand_paste.first().unwrap().to_string())),
            Line::from(""),
            Line::from(format!("{:012} - History previous item", self.kb_config.history_prev.first().unwrap().to_string())),
            Line::from(format!("{:012} - History next item", self.kb_config.history_next.first().unwrap().to_string())),
            Line::from(""),
            Line::from(format!("{:012} - Scroll up", self.kb_config.scroll_up.first().unwrap().to_string())),
            Line::from(format!("{:012} - Scroll down", self.kb_config.scroll_down.first().unwrap().to_string())),
            Line::from(format!("{:012} - Scroll page up", self.kb_config.scroll_up_page.first().unwrap().to_string())),
            Line::from(format!("{:012} - Scroll page down", self.kb_config.scroll_down_page.first().unwrap().to_string())),
            Line::from(""),
            Line::from(format!("{:012} - Scroll right", self.kb_config.scroll_right.first().unwrap().to_string())),
            Line::from(format!("{:012} - Scroll left", self.kb_config.scroll_left.first().unwrap().to_string())),
            Line::from(""),
            Line::from(format!("{:012} - Wrap output lines", self.kb_config.toggle_wrap.first().unwrap().to_string())),
        ]);

        Popup::new(lines)
            .title(" Keys ")
            .style(self.theme.popup)
            .render(area, buf);
    }
}
