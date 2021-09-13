use std::{{collections::HashMap}, io::{Error}, fs::{OpenOptions}};

fn main() {
    // Setting commands to be used to add todo from cli: 
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    // Initializing the db: 
    let mut todo = Todo::new().expect("Initialisation of db failed");

    // Handling commands: 
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!(">> TODO has been saved."),
            Err(why) => println!(">> An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list.", item),
            Some(_) => match todo.save() {
                Ok(_) => println!(">> TODO has been saved."),
                Err(why) => println!(">> An error occurred: {}", why),
            }
        }
    }
}

// Using HasMap to store key/value pairs: 
struct Todo {
    map: HashMap<String, bool>,
}
// In its implementation, 
impl Todo {
    fn new() -> Result<Todo, Error> {
        // Creating json file to hold data: 
        let db_file = OpenOptions::new().write(true).create(true).read(true).open("db.json")?; // Open the db file

        // Using serde_json to serialize json as HashMap: 
        match serde_json::from_reader(db_file) { 
            Ok(map) => Ok(Todo { map }),
            Err(why) if why.is_eof() => Ok (Todo { map: HashMap::new() }),
            Err(why) => panic!(">> An error occured: {}", why),
        }
    }
    // Function to insert a new item into the HashMap, passing default value as true: 
    fn insert(&mut self, key: String) {
        self.map.insert(key, true); 
    }
    // Function to save; opening db.json file and writing into it using serde_json: 
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(file, &self.map)?;
        Ok(())
    }
    // Function to mark todo complete: 
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
