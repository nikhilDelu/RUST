fn main() {
    // let ax = String::from("ILoveRust");
    // let char1 = ax.chars().nth(6);
    // match char1 {
    //     Some(c) => println!("char1: {}", c),
    //     None => print!("No Character"),
    // }
    // println!("char1: {}", char1.unwrap());
    let sentence = String::from("my name is Yash Bishnoi");
    let first_word = get_first_word(sentence);
    print!("First word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push_str(char.to_string().as_str());
    }
    return ans;
}

fn luup() {
    for i in 1..11 {
        print!("{i}");
    }
}
