use std::fs;
use std::path::Path;

fn main() {

 
    //let file_path = Path::new("src/resources/players.json");
    //let data = fs::read_to_string(file_path).expect("unable to read file");
    
    let mapper = Mapper::new();
    let data = mapper.load();

    println!("{}", data);



}

 struct Player {
    name: String,
    initiative: i8,
 }


 struct NPC {
    name: String,
    initiative: i8,
    parry: i8,
    health_points: i8,
    attack: i8,
    comment: String,
 }

 struct Mapper {
    file_path: String,
    players: Vec<Player>,
    npcs: Vec<NPC>,
 }

 impl Mapper {
    fn new() -> Self {
        Mapper {
            file_path: String::from("src/resources/players.json"),
            players: Vec::new(),
            npcs: Vec::new(),
        }
    }

    fn load(self: &self) -> String {

        let path: &Path = Path::new(self.file_path);

        fs::read_to_string(path).expect("unable to read file")
    }
 }