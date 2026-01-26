use crate::data_structures::bst::Command;
use crate::data_structures::bst::CommandAction;
use crate::data_structures::bst::CommandTree;
use crate::data_structures::linked_list::LinkedList;
use ratatui::{style::Color, widgets::ListState};

pub enum AppMode {
    Login,
    Shell,
}

pub struct AppState {
    pub mode: AppMode,
    pub input_text: String,
    pub history_list: LinkedList<String>,
    pub command_registry: CommandTree,
    pub master_password: String,
    pub status_message: String,
    pub message_color: Color,
    pub is_authenticated: bool,
    pub scroll_state: ListState,
    pub command_history: Vec<String>,
    pub history_index: usize,
}

impl AppState {
    pub fn new() -> Self {
        let mut tree = CommandTree::new();

        if tree.load_from_file("registry.json").is_err() {
            tree.insert(Command::new(
                "start",
                "Initialize core systems",
                CommandAction::DataDisplay,
            ));
            tree.insert(Command::new(
                "status",
                "Check system health",
                CommandAction::DataDisplay,
            ));
            tree.insert(Command::new(
                "clear",
                "Clear command history",
                CommandAction::SystemClear,
            ));
            tree.insert(Command::new(
                "ls",
                "List files in current directory",
                CommandAction::ExecuteCaptured,
            ));

            tree.insert(Command::new(
                "cd",
                "change the current directory",
                CommandAction::SystemCd,
            ));
            tree.insert(Command::new(
                "cat",
                "print file contect",
                CommandAction::ExecuteCaptured,
            ));
            tree.insert(Command::new(
                "man",
                "manual page for help",
                CommandAction::ExecuteInteractive,
            ));
            tree.insert(Command::new(
                "pwd",
                "print current working directory",
                CommandAction::ExecuteCaptured,
            ));
            tree.insert(Command::new(
                "reg",
                "Register a command",
                CommandAction::DataDisplay,
            ));
            tree.insert(Command::new(
                "del",
                "Delete a Registered command",
                CommandAction::DataDisplay,
            ));

            let _ = tree.save_to_file("registry.json");
        }

        Self {
            mode: AppMode::Login,
            input_text: String::new(),
            history_list: LinkedList::new(),
            command_registry: tree,
            master_password: String::from("radium77"),
            status_message: String::from("Awaiting Authentication..."),
            message_color: Color::White,
            is_authenticated: false,
            scroll_state: ListState::default(),
            command_history: Vec::new(),
            history_index: 0,
        }
    }
}
