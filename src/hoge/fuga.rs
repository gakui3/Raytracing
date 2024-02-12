pub struct Fuga {
    text: String,
}

impl Fuga {
    pub fn new(text: String) -> Self {
        Fuga { text }
    }

    pub fn print(&self) {
        println!("{}", self.text);
    }

    //文字列に"!"を追加して返す
    pub fn add_exclamation(&self) -> String {
        format!("{}!", self.text) //rustでは明示的にreturnを書かない。セミコロンを書かないとreturnされる
    }
}
