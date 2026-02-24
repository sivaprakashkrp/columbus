use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget, Wrap},
};

// HelpOverview Struct and render method copied from https://ratatui.rs/recipes/render/overwrite-regions/ and edited as per requirements of the Application

#[derive(Debug, Default)]
pub struct HelpOverview {
    pub title: String,
    pub content: String,
    pub border_style: Style,
    pub title_style: Style,
    pub style: Style,
    pub scroll: u16,
    pub max_scroll: u16,
}

impl HelpOverview {
    pub fn new() -> HelpOverview {
        let content_text = "
            Keybindings
            ===========

            Universal Keybindings
            ---------------------
            \th ->      Help Overview
            \ta ->      Input mode in the Path Field
            \t: ->      Input mode in Command bar
            \tTab ->    Used to cycle through the explorers and input fields
            \tq ->      Exits the app
                
            Explorer
            --------
            \tj | k ->      Up / Down selection
            \tr ->          Refresh the explorer
            \tc ->          Copy the selected file/directory
            \tv ->          Paste the copied file/directory
            \tx ->          Cut the selected file/directory
            \tdelete ->     Delete the selected item
            \tEnter :-
            \t\tOn File ->      Opens the file if the file extension is supported and defined in file_options.toml configuration file.
            \t\tOn Directory -> Navigates into the directory.
                    
            Path Field
            ----------
            \ta ->      Enter Input Mode (Characters typed are registered. All Keybindings are disabled.)
            \tEsc ->    Exit Input Mode
            \tEnter ->  Navigate to the provided PATH
            
            Drives Explorer
            ---------------
            \tj | k ->  Up / Down Selection
            \tEnter ->  Navigate into the selected drive
            \tr ->      Refresh the drives menu

            Command Bar
            -----------
            \ta ->      Enter Input Mode (Characters typed are registered. All Keybindings are disabled.)
            \tEsc ->    Exit Input Mode
            \tEnter ->  Execute the entered command with the given arguments

            Quick Access
            ------------
            \tj | k -> Up / Down Selection
            \tEnter -> Navigate into the selected Folder

            Commands for Command Bar
            ------------------------
            \tCreate a new file:        n <FILENAME>
            \tCreate a new directory:   b <DIRECTORY NAME>
            \tRename Operation:         r <OLD_EXISTING_NAME> <NEW_NAME>

            Help Overview
            -------------
            \tj | k -> Up / Down Navigation
            \tq -> Close Help Overview

            INFORMATION
            -----------
            Refer to the Github Repo https://github.com/sivaprakashkrp/columbus for more information or to report issues

            Thank You!!
        ";
        // let content_span_tags = ;
        HelpOverview {
            title: String::from(" Help Overview "),
            content: format!("{}", content_text),
            border_style: Style::new().cyan(),
            title_style: Style::new().cyan(),
            style: Style::default(),
            scroll: 0,
            // Excess scroll space for smaller displays
            max_scroll: 60,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title.clone())
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0))
            .border_style(self.border_style);
        Paragraph::new(self.content.clone())
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}
