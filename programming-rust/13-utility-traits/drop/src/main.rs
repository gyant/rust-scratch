struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

//trait Drop {
//    fn drop(&mut self);
//}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

fn main() {
    let mut a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };

    println!("before assignment");
    a = Appellation {
        name: "Hera".to_string(),
        nicknames: vec![],
    };
    println!("at end of block");
}
