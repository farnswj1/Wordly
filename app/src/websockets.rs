use std::{collections::HashSet, sync::LazyLock};

use axum::extract::ws::{Message, WebSocket};
use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use tracing::{error, info};

use crate::data::{UNUSED_WORDS, WORDS};

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z]{5}$").unwrap());

pub async fn handle_socket(mut socket: WebSocket, who: String) {
    let word = WORDS.choose(&mut thread_rng()).unwrap().to_string();
    info!("Word for {who} -> {word}");

    let mut num_attempts: u8 = 0;
    let mut letters = HashSet::new();

    for letter in word.chars() {
        letters.insert(letter);
    }

    while num_attempts < 6 {
        // Wait for a message from the client.
        let response = socket.recv().await;
        if let None = response {
            error!("{who} suddenly disconnected.");
            return;
        }

        // Handle a sudden disconnection.
        let message = response.unwrap();
        if let Err(error) = message {
            error!("{who} abruptly disconnected: {error}");
            return;
        }

        // Process the client's message.
        let msg = message.unwrap();
        if let Message::Text(guess) = msg {
            // The guess must be alphabetical and 5 letters long.
            if !RE.is_match(&guess) {
                let response = "invalid:The word must be 5 letters long.".to_string();
                if let Err(error) = socket.send(Message::Text(response)).await {
                    error!("Failed to send message to {who}: {error}");
                    return;
                }
                continue;
            }

            // The word must be in either list.
            let guess_as_str = &guess.as_str();
            if let Err(_) = WORDS.binary_search(guess_as_str) {
                if let Err(_) = UNUSED_WORDS.binary_search(guess_as_str) {
                    let response = "invalid:That is not a word.".to_string();
                    if let Err(error) = socket.send(Message::Text(response)).await {
                        error!("Failed to send message to {who}: {error}");
                        return;
                    }
                    continue;
                }
            }

            // Calculate the results by comparing the guess to the word.
            let mut result = String::from("result:");
            let zip = word.chars().zip(guess.chars());

            for (word_char, guess_char) in zip {
                if word_char == guess_char {
                    result.push(word_char);
                }
                else if letters.contains(&guess_char) {
                    result.push('*');
                }
                else {
                    result.push('-');
                }
            }

            // Send the result to the client.
            if let Err(error) = socket.send(Message::Text(result)).await {
                error!("Failed to send message to {who}: {error}");
                return;
            }

            // Close the socket if the user correctly guessed the word.
            // Otherwise, increment the number of attempts by 1.
            num_attempts += 1;

            if guess == word {
                info!("{who} correctly guessed the word in {num_attempts} attempt(s).");
                return;
            }
            else if num_attempts == 6 {
                // Send the words to the client if the client runs out the attempts.
                let response = format!("final:{}", word);
                if let Err(error) = socket.send(Message::Text(response)).await {
                    error!("Failed to send final message to {who}: {error}");
                }
                return;
            }
        }
        else if let Message::Close(close) = msg {
            // Need to handle close frame edge case conditionally.
            if let Some(cf) = close {
                info!("{} sent close with code {} and reason: `{}`", who, cf.code, cf.reason);
            } else {
                info!("{who} somehow sent close message without CloseFrame");
            }
            return;
        }
        else {
            error!("{who} sent a message that is not accepted by the server.");
            return;
        }
    }

    info!("{who} disconnected.");
}
