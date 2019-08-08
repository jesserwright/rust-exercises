pub fn pig_latin(word: &str) -> String {
  // Convert strings to pig latin. The first consonant of each word is moved to the end of the
  // word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay”
  // added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
  // think of the String type as a vector of the str type
  let vowels = ["a", "e", "i", "o", "u"];

  let mut word = String::from(word);

  let first_letter = &word[..1];

  for vowel in vowels.iter() {
    if first_letter == vowel.to_string() {
      word.push_str(&"-hay");
      break;
    } else {
      let first_removed = word.remove(0).to_string();
      word.push_str(&"-");
      word.push_str(&first_removed);
      word.push_str(&"ay");
      break;
    }
  }
  word // 🖐️🎤🔥
}
