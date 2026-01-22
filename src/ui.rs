use ratatui::{
    Frame, layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState}
};

use crate::{data_structures::bst::is_protected, stats::AppState};

pub fn draw_ui(f: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(f.area());

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(chunks[1]);

    let title = Paragraph::new("   RADIUM-SHELL v1.0.0 ")
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[0]);

    let messages: Vec<ListItem> = app_state.history_list
        .to_vec()
        .into_iter()
        .map(ListItem::new)
        .collect();

    let output_list = List::new(messages)
        .block(Block::default().title(" Output ").borders(Borders::ALL).border_type(BorderType::Rounded))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD)) 
        .highlight_symbol(" ->");

    f.render_stateful_widget(
        output_list, 
        middle_chunks[0], 
        &mut app_state.scroll_state.clone()
    );

    let mut scrollbar_state = ScrollbarState::new(app_state.history_list.size)
        .position(app_state.scroll_state.selected().unwrap_or(0));

    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        middle_chunks[0], 
        &mut scrollbar_state,
    );

    let mut commands_in_bst: Vec<ListItem> = app_state.command_registry
        .get_all_names()
        .into_iter()
        .map(|name| {
            if is_protected(&name) {
                ListItem::new(format!("  {} [SYS]", name))
                    .style(Style::default().fg(Color::DarkGray)) 
            } else {
                ListItem::new(format!("  {}", name))
                    .style(Style::default().fg(Color::Cyan))
            }
        })
        .collect();

    if app_state.input_text.starts_with('!') {
        let ghost_cmd = app_state.input_text.trim();
        commands_in_bst.insert(0, 
            ListItem::new(format!(" 󱐋 {} (ROOT)", ghost_cmd))
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        );
    }

    let sidebar = List::new(commands_in_bst)
        .block(Block::default()
            .title(" Registry ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded))
        .highlight_style(Style::default().bg(Color::Indexed(236)))
        .style(Style::default());

    f.render_widget(sidebar, middle_chunks[1]);

    let input_area = chunks[2];


    let prediction = if app_state.input_text.starts_with("!!") {
        app_state.command_registry.find_prediction(&app_state.input_text[2..])
    } else if app_state.input_text.starts_with("!") {
        app_state.command_registry.find_prediction(&app_state.input_text[1..])
    } else {
        app_state.command_registry.find_prediction(&app_state.input_text)
    };

    let display_text = if let Some(ref pred) = prediction {
        let pred_len = if app_state.input_text.starts_with("!!") {
            app_state.input_text.len() - 2
        } else if app_state.input_text.starts_with("!") {
            app_state.input_text.len() - 1
        } else {
            app_state.input_text.len()
        };

        Line::from(vec![
            Span::styled(&app_state.input_text, Style::default().fg(Color::Yellow)),
            Span::styled(&pred[pred_len..], Style::default().fg(Color::DarkGray)),
        ])
    } else {
        Line::from(Span::raw(&app_state.input_text))
    };

    let input_color = if app_state.input_text.starts_with('!') {
        Color::Red
    } else {
        Color::Cyan
    };

    let input_widget = Paragraph::new(display_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(input_color))
                .padding(Padding::left(2))
                .title(if app_state.input_text.starts_with('!') { " [ ROOT EXECUTION ] " } else { " [ Input (Use ! for Root, !! Interactive Root) ] " })
        );

    f.render_widget(input_widget, input_area);
    f.set_cursor_position((
        chunks[2].x + app_state.input_text.len() as u16 + 3,
        chunks[2].y + 1,
    ));

    let last_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[3]);

    let cwd = std::env::current_dir().unwrap_or_default();
    let cwd_display = format!("   {}", cwd.display());
    let cwd_widget = Paragraph::new(cwd_display)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(HorizontalAlignment::Left);

    let status_bar = Paragraph::new(app_state.status_message.as_str())
        .style(Style::default().fg(app_state.message_color))
        .alignment(HorizontalAlignment::Right);

    f.render_widget(cwd_widget, last_chunks[0]);
    f.render_widget(status_bar, last_chunks[1]);
}

pub fn draw_login(f: &mut Frame, app_state: &AppState) {
    let area = centered_rect(40, 8, f.area()); 

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);

    f.render_widget(
        Block::default()
            .title(" SYSTEM LOCK ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Red))
        ,
        area
    );

    let masked_password: String = "∗".repeat(app_state.input_text.len());
    let input_box = Paragraph::new(masked_password)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Enter Key ")
            .padding(Padding::left(1))
        )
        .style(Style::default().fg(Color::Yellow));
    
    f.render_widget(input_box, chunks[1]);

    f.set_cursor_position((
        chunks[1].x + 2 + app_state.input_text.len() as u16,
        chunks[1].y + 1,
    ));

    let status_bar = Paragraph::new(app_state.status_message.as_str())
        .style(Style::default().fg(app_state.message_color))
        .alignment(HorizontalAlignment::Right);

    f.render_widget(status_bar, chunks[2]);
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let vertical_pad = (r.height.saturating_sub(height)) / 2;
    
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_pad),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
