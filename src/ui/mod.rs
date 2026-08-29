/*
    WARNING:

    This file is heavily done by Google Gemini since i never used ratatui and I knew it will take a while.
    Therefore, there might be some issues hiding somewhere I didn't check.
    Tho I've read the entire code, I've maybe gotten skill issued.
*/

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use std::io;

use crate::Command;
use crate::command::{Placeholder, extract_placeholders};

enum AppState {
    Browsing,
    Prompting {
        command_template: String,
        placeholders: Vec<Placeholder>,
        current_step: usize,
        text_input: String,
        option_state: ListState,
    },
}

pub fn run(commands: &[Command]) -> io::Result<Option<String>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ListState::default();
    state.select(Some(0));

    let mut app_state = AppState::Browsing;

    let result = loop {
        terminal.draw(|f| {
            render_main_list(f, commands, &mut state);

            if let AppState::Prompting {
                placeholders,
                current_step,
                text_input,
                option_state,
                ..
            } = &mut app_state
                && let Some(placeholder) = placeholders.get(*current_step)
            {
                render_prompt_modal(
                    f,
                    placeholder,
                    text_input,
                    option_state,
                    *current_step,
                    placeholders.len(),
                );
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match &mut app_state {
                AppState::Browsing => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(None),
                    KeyCode::Down | KeyCode::Char('j') => {
                        let next = match state.selected() {
                            Some(i) => {
                                if i >= commands.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(next));
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        let prev = match state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    commands.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(prev));
                    }
                    KeyCode::Enter => {
                        if let Some(selected_idx) = state.selected() {
                            let cmd = &commands[selected_idx];
                            let placeholders = extract_placeholders(&cmd.command);

                            if placeholders.is_empty() {
                                break Ok(Some(cmd.command.clone()));
                            } else {
                                let mut opt_state = ListState::default();
                                opt_state.select(Some(0));
                                app_state = AppState::Prompting {
                                    command_template: cmd.command.clone(),
                                    placeholders,
                                    current_step: 0,
                                    text_input: String::new(),
                                    option_state: opt_state,
                                };
                            }
                        }
                    }
                    _ => {}
                },

                AppState::Prompting {
                    command_template,
                    placeholders,
                    current_step,
                    text_input,
                    option_state,
                } => {
                    let current_p = &placeholders[*current_step];

                    match key.code {
                        KeyCode::Esc => {
                            app_state = AppState::Browsing;
                        }
                        _ if current_p.options.is_some() => {
                            let options = current_p.options.as_ref().unwrap();
                            let total_options = if !current_p.required {
                                options.len() + 1
                            } else {
                                options.len()
                            };

                            match key.code {
                                KeyCode::Down | KeyCode::Char('j') => {
                                    let next = match option_state.selected() {
                                        Some(i) if i >= total_options - 1 => 0,
                                        Some(i) => i + 1,
                                        None => 0,
                                    };
                                    option_state.select(Some(next));
                                }
                                KeyCode::Up | KeyCode::Char('k') => {
                                    let prev = match option_state.selected() {
                                        Some(0) => total_options - 1,
                                        Some(i) => i - 1,
                                        None => 0,
                                    };
                                    option_state.select(Some(prev));
                                }
                                KeyCode::Enter => {
                                    let chosen_value = match option_state.selected() {
                                        Some(i) => {
                                            if !current_p.required {
                                                if i == 0 {
                                                    String::new()
                                                } else {
                                                    options[i - 1].clone()
                                                }
                                            } else {
                                                options[i].clone()
                                            }
                                        }
                                        None => String::new(),
                                    };

                                    *command_template =
                                        command_template.replace(&current_p.raw, &chosen_value);
                                    *current_step += 1;
                                    *text_input = String::new();
                                    option_state.select(Some(0));

                                    if *current_step >= placeholders.len() {
                                        break Ok(Some(command_template.clone()));
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Char(c) => {
                            text_input.push(c);
                        }
                        KeyCode::Backspace => {
                            text_input.pop();
                        }
                        KeyCode::Enter => {
                            if current_p.required && text_input.trim().is_empty() {
                            } else {
                                *command_template =
                                    command_template.replace(&current_p.raw, text_input.trim());
                                *current_step += 1;
                                *text_input = String::new();
                                option_state.select(Some(0));

                                if *current_step >= placeholders.len() {
                                    break Ok(Some(command_template.clone()));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn render_main_list(f: &mut Frame, commands: &[Command], state: &mut ListState) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let inner_width = chunks[0].width.saturating_sub(4) as usize;

    let items: Vec<ListItem> = commands
        .iter()
        .map(|cmd| {
            let name_str = format!("{:<16}", cmd.name);
            let cmd_str = &cmd.command;

            let path_str = cmd
                .file_path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| cmd.file_path.to_string_lossy().to_string());

            let left_len = name_str.chars().count() + 3 + cmd_str.chars().count();
            let right_len = path_str.chars().count();

            let spaces_needed = inner_width.saturating_sub(left_len + right_len);
            let spacing = " ".repeat(spaces_needed);

            let text = Line::from(vec![
                Span::styled(
                    name_str,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" │ "),
                Span::styled(cmd_str, Style::default().fg(Color::Yellow)),
                Span::raw(spacing),
                Span::styled(path_str, Style::default().fg(Color::DarkGray)),
            ]);

            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Commands "))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 40, 40))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[0], state);

    let help_text = Paragraph::new("↑/k: Up  |  ↓/j: Down  |  Enter: Select  |  q/Esc: Quit")
        .block(Block::default().borders(Borders::ALL).title(" Help "));
    f.render_widget(help_text, chunks[1]);
}

fn render_prompt_modal(
    f: &mut Frame,
    p: &Placeholder,
    text_input: &str,
    option_state: &mut ListState,
    step: usize,
    total_steps: usize,
) {
    let area = centered_rect(60, 35, f.area());
    f.render_widget(Clear, area);

    let req_label = if p.required {
        Span::styled(" (Required)", Style::default().fg(Color::Red))
    } else {
        Span::styled(" (Optional)", Style::default().fg(Color::DarkGray))
    };

    let title = Line::from(vec![
        Span::styled(
            format!(" [{}/{}] Variable: ", step + 1, total_steps),
            Style::default().fg(Color::White),
        ),
        Span::styled(
            &p.name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        req_label,
        Span::raw(" "),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().bg(Color::Rgb(20, 20, 20)));

    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    if let Some(options) = &p.options {
        let mut items: Vec<ListItem> = Vec::new();

        if !p.required {
            items.push(ListItem::new(Span::styled(
                "  (Skip / Empty)",
                Style::default().fg(Color::DarkGray),
            )));
        }

        items.extend(
            options
                .iter()
                .map(|opt| ListItem::new(format!("  {}", opt))),
        );

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Rgb(50, 50, 50))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(list, inner_area, option_state);
    } else {
        let text_layout = Layout::default()
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(inner_area);

        let input_widget = Paragraph::new(format!("{}_", text_input))
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Type value and press Enter "),
            );

        f.render_widget(input_widget, text_layout[1]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
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
