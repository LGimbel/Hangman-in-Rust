
use random_word::Lang;
use std::io;
fn main() {
    println!("Welcome to hangman\nPlease enter how long you want the word to be 2-15 letters.13");
    let mut word_length = String::new();
    let mut _total_guesses = 0;
    let mut _incorect_guesses = 0;
    io::stdin().
        read_line(&mut word_length).expect("failed");
    let word_length: usize = word_length.trim().parse().expect("not a number you bafoon!");
    let rand_word = random_word::gen_len(word_length, Lang::En);
    println!("{:?}",rand_word);
    let mut correct_answer:Vec<char>= vec![];
    if let Some(correct_string) = rand_word{
        for c in correct_string.chars(){
            correct_answer.push(c);
            
        }
    }
    println!("{:?}",correct_answer);
    
    let mut player_answers = vec!['-';word_length];
    println!("{:?}",player_answers);
    player_guess(&mut player_answers, correct_answer);

}
fn player_guess(player_answers: &mut Vec<char>,correct_answer:Vec<char>){
    let mut current_guess = Default::default();
    let answer_correct_tup:(&mut Vec<char>,bool);
    io::stdin().
        read_line(&mut current_guess).
        expect("Guess failed");
        current_guess = current_guess.to_lowercase().trim().to_string();
        let guess_letter_count = current_guess.chars().count();
    if guess_letter_count == 1{
        println!("the guess was only one letter");
       // println!("{:?}",player_answers);
       answer_correct_tup = individual_guess(player_answers, correct_answer, current_guess);
       println!("{:?} g:{:?}",answer_correct_tup.0,answer_correct_tup.1);
      
      
        
    }else {
        println!("the guess was more than one letter");
        let player_win = word_guess(correct_answer, current_guess);
        if player_win{
            println!("Congrats you won and correctly guessed the word");
            //call game end stats.
        }
    };


   
}
 fn individual_guess(player_answer: &mut Vec<char>,correct_answer: Vec<char>,guess:String) ->(&mut Vec<char>,bool){
    let mut guess_correct = false;
    let guessed_character:Vec<char> = guess.chars().collect();
    let guessed_character = guessed_character[0];
    for (i,c) in correct_answer.into_iter().enumerate(){
        if guessed_character == c{
            player_answer[i] = c;
            guess_correct = true;
        }
    }
    println!("{:?}",player_answer);
    (player_answer,guess_correct)    
    }
fn word_guess(correct_answer: Vec<char>,guess:String) -> bool{
    let correct_string:String = correct_answer.iter().collect();
    println!("{correct_string}");
    if guess.chars().count() == correct_answer.len(){
        println!("char count matches");
        correct_string == guess
    }
    else {
        false
    }
}
fn _player_win(correct_answer: Vec<char>,player_answer: Vec<char>) -> bool{
    correct_answer == player_answer
}

