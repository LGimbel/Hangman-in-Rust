
use random_word::Lang;
use std::io;
fn main() {
    println!("Welcome to hangman\nPlease enter how long you want the word to be 2-15 letters.13");
    let mut word_length = String::new();
    let mut total_guesses = 0;
    let mut incorect_guesses = 0;
    //should probbly be stored as a tupple but what ever.
    io::stdin().
        read_line(&mut word_length).expect("failed");
    let word_length: usize = word_length.trim().parse().expect("not a number you bafoon!");
    let rand_word = random_word::gen_len(word_length, Lang::En);
    let mut correct_answer:Vec<char>= vec![];
    if let Some(correct_string) = rand_word{
        for c in correct_string.chars(){
            correct_answer.push(c);
            
        }
    }
    
    let mut player_answers = vec!['_';word_length];
    println!("{:?}",player_answers);
    while !player_win(correct_answer.clone(), &mut player_answers) {
        player_guess(&mut player_answers, correct_answer.clone(),&mut total_guesses,&mut incorect_guesses);
        println!("{}",player_answers.iter().collect::<String>());
    }
    end_stats(correct_answer, &mut incorect_guesses, &mut total_guesses)

    

}
fn player_guess(player_answers: &mut Vec<char>,correct_answer:Vec<char>,total_guesses:&mut i32,incorect_guesses: &mut i32){
    let mut current_guess = Default::default();
    let answer_correct_tup:(&mut Vec<char>,bool);
    io::stdin().
        read_line(&mut current_guess).
        expect("Guess failed");
        current_guess = current_guess.to_lowercase().trim().to_string();
        let guess_letter_count = current_guess.chars().count();
    if guess_letter_count == 1{
       answer_correct_tup = individual_guess(player_answers, correct_answer.clone(), current_guess);
       if !answer_correct_tup.1{
        *incorect_guesses +=1;
       }
       *total_guesses +=1;
      
      
        
    }else {
        *total_guesses+=1;
        let player_win = word_guess(correct_answer.clone(), current_guess);
        if !player_win{ 
             println!("Not quite :( keep trying");
            *incorect_guesses +=1;
        }
        else {
            end_stats(correct_answer, incorect_guesses, total_guesses)
        }
    };

}
 fn individual_guess(player_answer: &mut Vec<char>,correct_answer: Vec<char>,guess:String) ->(&mut Vec<char>,bool){
    let mut guess_correct = false;
    let guessed_character:Vec<char> = guess.chars().collect();
    let guessed_character = guessed_character[0];
    for (i,c) in (*correct_answer).iter().enumerate(){
        if guessed_character == *c{
            player_answer[i] = *c;
            guess_correct = true;
        }
    }
    (player_answer,guess_correct)    
    }
fn word_guess(correct_answer: Vec<char>,guess:String) -> bool{
    let correct_string:String = correct_answer.iter().collect();
    if guess.chars().count() == correct_answer.len(){
        correct_string == guess
    }
    else {
        false
    }
}
fn player_win(correct_answer: Vec<char>,player_answer: &mut Vec<char>) -> bool{
    correct_answer == *player_answer
}
fn end_stats(correct_answer: Vec<char>,incorect_guesses: &mut i32,total_guesses:&mut i32){
    let correct_answer:String = correct_answer.iter().collect();
    let accuracy = ((*total_guesses as f32 - *incorect_guesses as f32)/ *total_guesses as f32) * 100.0;
    println!("The word was: \"{correct_answer}\"");
    println!("you had a total of {total_guesses} and {incorect_guesses} were wrong meaning that you guess acuracy is {accuracy}%");
    std::process::exit(0);
}
