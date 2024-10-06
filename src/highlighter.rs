// src/highlighter.rs
use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;
use crate::token_type::token_types::TokenTypes;

#[derive(Debug, Clone)]
pub struct HighlightStyle {
    pub foreground: String,
    pub background: String,
    pub font_style: Option<String>, // e.g., "bold", "italic"
}

pub fn get_highlighting_style(token: &TokenTypes) -> HighlightStyle {
    match token {
        TokenTypes::Int => HighlightStyle {
            foreground: String::from("blue"),
            background: String::from(""),
            font_style: None,
        },
        TokenTypes::String => HighlightStyle {
            foreground: String::from("green"),
            background: String::from(""),
            font_style: None,
        },
        TokenTypes::Char => HighlightStyle {
            foreground: String::from("purple"),
            background: String::from(""),
            font_style: None,
        },
        TokenTypes::AssignmentOperator => HighlightStyle {
            foreground: String::from("red"),
            background: String::from(""),
            font_style: None,
        },
        TokenTypes::Bool => HighlightStyle {
            foreground: String::from("cyan"),
            background: String::from(""),
            font_style: None,
        },
        TokenTypes::Function { .. } => HighlightStyle {
            foreground: String::from("orange"),
            background: String::from(""),
            font_style: Some(String::from("bold")),
        },
        TokenTypes::Comment => HighlightStyle {
            foreground: String::from("gray"),
            background: String::from(""),
            font_style: Some(String::from("italic")),
        },
        _ => HighlightStyle {
            foreground: String::from("black"),
            background: String::from(""),
            font_style: None,
        },
    }
}

pub fn highlight_code(source: &str) -> Vec<(String, HighlightStyle)> {
    let tokens = tokenize(source.to_string()); // Replace this with your actual tokenizer call
    let mut highlighted_code = Vec::new();

    for info in tokens {
        let token = info.token;
        let style = get_highlighting_style(&token);
        highlighted_code.push((token.to_string(), style)); // Store the token with its highlighting style
    }

    highlighted_code
}

use ansi_term::Colour;

pub fn display_highlighted_code(highlighted_code: Vec<(String, HighlightStyle)>) {
    for (token, style) in highlighted_code {
        let mut formatted_token = token.clone();

        // Apply foreground color
        let foreground_color = match style.foreground.as_str() {
            "blue" => Colour::Blue,
            "green" => Colour::Green,
            "purple" => Colour::Purple,
            "red" => Colour::Red,
            "cyan" => Colour::Cyan,
            "orange" => Colour::Yellow, // ANSI does not have an orange, using yellow instead
            "gray" => Colour::Fixed(245), // A gray color
            _ => Colour::White,         // Default color
        };

        // Apply font styles if necessary
        if let Some(font_style) = style.font_style.as_deref() {
            if font_style == "bold" {
                formatted_token = format!("{}", foreground_color.bold().paint(&token.clone()));
            } else if font_style == "italic" {
                formatted_token = format!("{}", foreground_color.italic().paint(&token.clone()));
            }
        } else {
            formatted_token = format!("{}", foreground_color.paint(&token.clone()));
        }

        // Print the formatted token
        print!("{}", formatted_token);
    }

    // New line after printing all tokens
    println!();
}
