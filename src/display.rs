use crate::frames::{Frame, AnimatedFrames};
use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout, Write};
use std::time::{Duration, Instant};

const ANIMATION_WIDTH: u16 = 64;
const ANIMATION_HEIGHT: u16 = 29;
const MIN_TERMINAL_WIDTH: u16 = 72;
const MIN_TERMINAL_HEIGHT: u16 = 30;

pub fn create_speech_bubble_with_tail(text: &str, max_width: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + word.len() < max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    let bubble_width = lines.iter().map(|line| line.len()).max().unwrap_or(0).max(1);
    let mut bubble = Vec::new();
    
    // Top border
    bubble.push(format!("┌{}┐", "─".repeat(bubble_width + 2)));
    
    // Content lines
    for line in &lines {
        bubble.push(format!("│ {line:<bubble_width$} │"));
    }
    
    // Bottom border
    bubble.push(format!("└{}┘", "─".repeat(bubble_width + 2)));
    
    // Add tail pointing to Momoi (left side) using / characters
    // Add connecting lines with / pointing toward Momoi
    bubble.push("   /".to_string());
    bubble.push("  /".to_string());
    bubble.push(" /".to_string());
    
    bubble
}

pub fn display_say_command(frame: &Frame, text: &str) {
    let bubble = create_speech_bubble_with_tail(text, 30);
    let frame_lines = &frame.lines;
    
    let max_frame_height = frame_lines.len();
    let max_bubble_height = bubble.len();
    let max_height = max_frame_height.max(max_bubble_height);
    
    for i in 0..max_height {
        let frame_line = if i < frame_lines.len() {
            frame_lines[i]
        } else {
            ""
        };
        
        let bubble_line = if i < bubble.len() {
            &bubble[i]
        } else {
            ""
        };
        
        println!("{frame_line} {bubble_line}");
    }
}

pub fn check_terminal_size() -> io::Result<bool> {
    let (width, height) = terminal::size()?;
    Ok(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT)
}

pub fn setup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

pub fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    terminal::disable_raw_mode()?;
    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}

pub fn check_for_exit_input() -> io::Result<bool> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Esc => return Ok(true),
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => return Ok(true),
                _ => {}
            }
        }
    }
    Ok(false)
}

pub fn display_animation_once(frames: &AnimatedFrames, text: Option<&str>) -> io::Result<bool> {
    let bubble = text.map(|t| create_speech_bubble_with_tail(t, 30));
    let (term_width, term_height) = terminal::size()?;
    let mut stdout = stdout();
    
    for (current_frame, interval) in frames.iter() {
        // Check for user input at the start of each frame
        if check_for_exit_input()? {
            return Ok(true); // Exit requested
        }
        
        // Clear screen
        execute!(stdout, Clear(ClearType::All))?;
        
        // Calculate display area
        let total_width = if let Some(ref bubble) = bubble {
            let bubble_width = bubble.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
            ANIMATION_WIDTH + bubble_width + 2 // 2 for spacing
        } else {
            ANIMATION_WIDTH
        };
        
        let total_height = ANIMATION_HEIGHT;
        
        let start_x = (term_width.saturating_sub(total_width)) / 2;
        let start_y = (term_height.saturating_sub(total_height)) / 2;
        
        // Display current frame
        for (i, line) in current_frame.lines.iter().enumerate() {
            execute!(stdout, MoveTo(start_x, start_y + i as u16), Print(line))?;
        }
        
        // Display bubble if present
        if let Some(ref bubble) = bubble {
            let bubble_start_x = start_x + ANIMATION_WIDTH + 2;
            let bubble_start_y = start_y + (ANIMATION_HEIGHT.saturating_sub(bubble.len() as u16)) / 2;
            
            for (i, line) in bubble.iter().enumerate() {
                execute!(stdout, MoveTo(bubble_start_x, bubble_start_y + i as u16), Print(line))?;
            }
        }
        
        stdout.flush()?;
        
        // Wait for the frame duration
        let frame_duration = Duration::from_millis(interval);
        let start_time = Instant::now();
        
        // Check for input during frame display
        while start_time.elapsed() < frame_duration {
            if check_for_exit_input()? {
                return Ok(true); // Exit requested
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    
    Ok(false) // No exit requested
} 