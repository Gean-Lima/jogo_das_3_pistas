use std::io::{self, Write};
use languages::Languages;
use user_response::UserResponse;

pub mod languages;
pub mod user_response;

fn main() {
    clear();
    let mut languages = Languages::new();

    languages.languages.sort();

    let mut responses: Vec<UserResponse> = Vec::new();
    let mut total_score: u8 = 0;
    let mut give_up = false;

    for lang in languages.languages {
        let mut score_round: u8 = 15;
        let mut clue2 = false;
        let mut clue3 = false;

        loop {
            println!("Valendo {score_round} pontos, Qual é a linguagem?\n");
            println!("Dicas:\n");
            println!("[15 Pontos] {}", lang.clues[0]);
            println!("[10 Pontos] {}", if clue2 { lang.clues[1].to_string() } else { "*".repeat(15) });
            println!("[ 5 Pontos] {}\n", if clue3 { lang.clues[2].to_string() } else { "*".repeat(15) });

            println!("1) Informar resposta");
            println!("2) Mostrar próxima dica");
            println!("3) Desistir\n");

            print!("Escolha uma opção: ");

            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input_number: u8 = match input.trim().parse() {
                Ok(value) => if value >= 1 && value <= 3 { value }
                    else { clear(); continue },
                Err(_) => { clear(); continue }
            };

            match input_number {
                1 => {
                    let mut input_word = String::new();

                    print!("Digite: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut input_word).unwrap();

                    if input_word.trim().to_lowercase() == lang.name.to_lowercase() {
                        total_score += score_round;
                        responses.push(UserResponse {
                            response: input_word.trim().to_string(),
                            score: score_round
                        });
                        clear();
                        println!("Resposta correta! +{score_round} pontos");
                        println!("Próxima fase");
                        break;
                    }

                    clear();
                    println!("Resposta incorreta! Tente novamente");
                    continue;
                },
                2 => {
                    clear();

                    if clue2 && clue3 {
                        println!("*** Todas as dicas já foram exibidas ***");
                        continue;
                    }

                    if !clue2 {
                        clue2 = true;
                        score_round = 10;
                        continue;
                    }
                    
                    clue3 = true;
                    score_round = 5;
                    continue;
                }
                _ => {
                    give_up = true;
                    break;
                }
            }
        }

        if give_up {
            break;
        }
    }

    clear();

    println!("Suas respostas:");

    for response in responses {
        println!("{:.<15}{}", response.response, response.score);
    }

    println!("Total de pontos feitos: {total_score}");
}

fn clear() {
    print!("{esc}c", esc = 27 as char);
}
