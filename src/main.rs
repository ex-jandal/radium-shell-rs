use crate::{
    data_structures::bst::{Command, CommandAction, is_protected},
    stats::{AppMode, AppState},
    ui::{draw_login, draw_ui},
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend, style::Color};
use std::io::{self, Stdout};
use std::process::Command as OsCommand;

mod data_structures;
mod stats;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app_state = AppState::new();

    loop {
        terminal.draw(|f| match app_state.mode {
            AppMode::Login => draw_login(f, &app_state),
            AppMode::Shell => draw_ui(f, &app_state),
        })?;

        if event::poll(std::time::Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            if key.code == KeyCode::Esc {
                break;
            }

            if key.kind == event::KeyEventKind::Press {
                match app_state.mode {
                    AppMode::Login => handle_login_input(&mut app_state, key),
                    AppMode::Shell => handle_shell_input(&mut app_state, key),
                }
            }
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn handle_shell_input(app: &mut AppState, key: KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match key.code {
        KeyCode::Up => {
            if !app.command_history.is_empty() && app.history_index > 0 {
                app.history_index -= 1;
                app.input_text = app.command_history[app.history_index].clone();
            }
        }
        KeyCode::Down => {
            if !app.command_history.is_empty() && app.history_index < app.command_history.len() - 1
            {
                app.history_index += 1;
                app.input_text = app.command_history[app.history_index].clone();
            } else {
                app.history_index = app.command_history.len();
                app.input_text.clear();
            }
        }
        KeyCode::Tab => {
            if let Some(pred) = app.command_registry.find_prediction(&app.input_text) {
                app.input_text = pred;
            }
        }
        KeyCode::Char(c) => app.input_text.push(c),
        KeyCode::Backspace => {
            app.input_text.pop();
        }
        KeyCode::Enter => execute_command(app),
        KeyCode::PageUp => {
            let current_idx = app.scroll_state.selected().unwrap_or(0);
            let new_idx = current_idx.saturating_sub(1);
            app.scroll_state.select(Some(new_idx));
        }
        KeyCode::PageDown => {
            let current_idx = app.scroll_state.selected().unwrap_or(0);
            let max_idx = app.history_list.size.saturating_sub(1);
            let new_idx = (current_idx + 1).min(max_idx);
            app.scroll_state.select(Some(new_idx));
        }
        _ => {}
    }
}

fn handle_login_input(app: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.input_text.push(c);
        }
        KeyCode::Backspace => {
            app.input_text.pop();
        }
        KeyCode::Enter => {
            if app.input_text == app.master_password {
                app.mode = AppMode::Shell;
                app.status_message = "Type 'help' to start.".to_string();
                app.message_color = Color::Green;
            } else {
                app.status_message = "INVALID KEY".to_string();
                app.message_color = Color::Red;
            }
            app.input_text.clear();
        }
        _ => {}
    }
}

pub fn execute_command(app: &mut AppState) {
    if app.input_text.trim().is_empty() {
        return;
    }

    app.command_history.push(app.input_text.clone());
    app.history_index = app.command_history.len();

    let parts: Vec<&str> = app.input_text.split_whitespace().collect();

    let cmd_name = parts[0].to_lowercase();
    let args = &parts[1..];

    if cmd_name == "reg" && args.len() >= 2 {
        let name = args[0].to_string();
        let desc = args[1..].join(" ");
        let new_cmd = Command::new(&name, &desc, CommandAction::ExecuteOS);

        app.command_registry.insert(new_cmd);
        app.history_list.append(format!("[+] Registered: {}", name));
        app.history_list.append("\n".to_string());
        app.input_text.clear();

        let total_items = app.history_list.size;
        if total_items > 0 {
            app.scroll_state.select(Some(total_items - 1));
        }

        let _ = app.command_registry.save_to_file("registry.json");
        return;
    }

    if cmd_name == "del" && !args.is_empty() {
        let target = args[0].to_lowercase();

        if !is_protected(&target) {
            app.command_registry.delete(&target);
            app.history_list
                .append(format!("[-] Deleted command: {}", target));
            let _ = app.command_registry.save_to_file("registry.json");
        } else {
            app.history_list
                .append(format!("[!] Command is protected: {}", target));
        }

        app.history_list.append("\n".to_string());

        let total_items = app.history_list.size;
        if total_items > 0 {
            app.scroll_state.select(Some(total_items - 1));
        }

        app.input_text.clear();
        return;
    }

    if cmd_name == "help" {
        app.history_list.append("─".repeat(45));
        app.history_list
            .append(format!("{:<10} | {:<20}", "COMMAND", "DESCRIPTION"));
        app.history_list.append("─".repeat(45));

        for name in app.command_registry.get_all_names() {
            if let Some(c) = app.command_registry.search(name) {
                app.history_list
                    .append(format!("{:<10} | {:<20}", c.name, c.description));
            }
        }
        app.history_list.append("─".repeat(45));
        app.input_text.clear();
        return;
    }

    if let Some(cmd) = app.command_registry.search(cmd_name.to_string()) {
        match cmd.action {
            CommandAction::SystemCd => {
                let target_dir = args.first().unwrap_or(&".");
                if let Err(e) = std::env::set_current_dir(target_dir) {
                    app.history_list
                        .append(format!("[!] Error changing dir: {}", e));
                    app.history_list.append("\n".to_string());
                } else {
                    let new_path = std::env::current_dir().unwrap();
                    app.history_list
                        .append(format!("Moved to: {}", new_path.display()));
                    app.status_message = format!("CWD: {}", new_path.display());
                    app.message_color = Color::Green;
                    app.history_list.append("\n".to_string());
                }
            }
            CommandAction::ExecuteOS => {
                let full_cmd = format!("{} {}", cmd_name, args.join(" "));
                let result = run_os_command(&full_cmd);
                app.history_list
                    .append(format!("[Command]: '{}'", full_cmd.trim()));
                for line in result.lines() {
                    app.history_list.append(format!("  {}", line));
                }
                app.history_list.append("\n".to_string());
            }
            CommandAction::SystemClear => {
                app.history_list.purge();
                app.message_color = Color::Green;
            }
            CommandAction::SystemExit => std::process::exit(0),
            CommandAction::DataDisplay => {
                app.history_list
                    .append(format!("{}: {}", cmd.name, cmd.description));
                app.history_list.append("\n".to_string());
            }
        }
    } else {
        app.history_list
            .append(format!("[!] '{}' not in Registry", cmd_name));
        app.status_message = "use `reg` to add commands.".to_string();
        app.message_color = Color::Cyan;
        app.history_list.append("\n".to_string());
    }

    let total_items = app.history_list.size;
    if total_items > 0 {
        app.scroll_state.select(Some(total_items - 1));
    }

    app.input_text.clear();
}

fn run_os_command(cmd_name: &str) -> String {
    let output = OsCommand::new("sh").arg("-c").arg(cmd_name).output();

    match output {
        Ok(out) => {
            String::from_utf8_lossy(&out.stdout).to_string()
        }
        Err(e) => format!("Failed to execute: {}", e),
    }

}
