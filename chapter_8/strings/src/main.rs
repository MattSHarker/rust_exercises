fn main() {
    // creating a new empty string
    let mut s = String::new();


    // creating a string with initial data 
    // method 1
    let data = "the quick brown fox";
    let s = data.to_string();

    // method 2
    let s = "the quick brown fox".to_string();

    // method 3
    let s = String::from("The quick brown fox");


    // Strings are UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    // appending to a string
    let mut s = String::from("The quick brown fox");
    s.push_str(" jumped over the lazy dogs");
    println!("{}", s);
    // .push() can only push 1 char


    // String::from does not take ownership
    let mut s1 = String::from("123");
    let s2 = "456";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);


    // + can be used to concatenate strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 has been moved and can not be used anymore
    println!("s3: {}", s3);


    // how to add multiple things together cleanly with format!()
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    // format works like println!, except it returns a string instead of 
        // printing to the console
    

    // indexing into strings
    let s = "012346789";
    // let h = s[0];   // this is not allowed, due to how UTF-8 is stored

    // how UTF-8 is stored:
    // some characters take more than 1 byte of storage space
    let len1 = String::from("Hello").len();     // each letter is 1 bytes
    let len2 = String::from("안녕하세요").len(); // each letter is 3 bytes
    println!("len1: {}", len1);
    println!("len2: {}", len2);
    
    // since some letters use multiple bytes, indexing with [] is not allowed


    // string slices
    // for strings with letters that are multiple bytes, make sure the slice
        // does not access parts of letters
    let hello = String::from("안녕하세요");

    let s = &hello[0..6];   // okay!
    println!("{}", s);

    // let s = &hello[0..7];   // not okay :(
    

    // iterating over strings
    let hello = "नमस्ते";
    for c in hello.chars() {    // getting each char
        print!("{} ", c);
    }

    println!();
    for c in hello.bytes() {    // getting each byte
        print!("{} ", c);
    }
}
