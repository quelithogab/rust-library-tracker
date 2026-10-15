use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// Custom Enum for Book Genre
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum Genre {
    Fiction,
    NonFiction,
    Science,
    History,
    Technology,
}

impl Genre {
    fn from_str(input: &str) -> Option<Genre> {
        match input.trim().to_lowercase().as_str() {
            "fiction" => Some(Genre::Fiction),
            "nonfiction" | "non-fiction" => Some(Genre::NonFiction),
            "science" => Some(Genre::Science),
            "history" => Some(Genre::History),
            "technology" | "tech" => Some(Genre::Technology),
            _ => None,
        }
    }
}

// Custom Enum for Circulation Status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum Status {
    Available,
    CheckedOut,
}

// Book Struct representing individual library items
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Book {
    id: usize,
    title: String,
    author: String,
    genre: Genre,
    status: Status,
}

// Library Struct managing collections, persistence, and indexing
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Library {
    books: Vec<Book>,
    #[serde(skip)]
    genre_index: HashMap<String, Vec<usize>>,
}

impl Library {
    // Initializes a new library instance
    fn new() -> Self {
        Library {
            books: Vec::new(),
            genre_index: HashMap::new(),
        }
    }

    // Adds a new book to the collection and updates indices
    fn add_book(&mut self, title: String, author: String, genre: Genre) {
        let id = self.books.len() + 1;
        let book = Book {
            id,
            title: title.clone(),
            author,
            genre: genre.clone(),
            status: Status::Available,
        };

        self.books.push(book);
        self.rebuild_index();
        println!("\nSuccessfully added '{}' to your library!", title);
    }

    // Displays all books currently registered in the database
    fn list_books(&self) {
        if self.books.is_empty() {
            println!("\nYour library is currently empty.");
            return;
        }

        println!("\n================== LIBRARY COLLECTION ==================");
        for book in &self.books {
            let status_str = match book.status {
                Status::Available => "Available",
                Status::CheckedOut => "Checked Out",
            };
            println!(
                "ID: {} | Title: {} | Author: {} | Genre: {:?} | Status: {}",
                book.id, book.title, book.author, book.genre, status_str
            );
        }
        println!("=========================================================");
    }

    // Toggles a book's check-in / check-out status based on its ID
    fn toggle_status(&mut self, id: usize) {
        if let Some(book) = self.books.iter_mut().find(|b| b.id == id) {
            book.status = match book.status {
                Status::Available => {
                    println!("\nBook '{}' has been checked out.", book.title);
                    Status::CheckedOut
                }
                Status::CheckedOut => {
                    println!("\nBook '{}' has been returned/checked in.", book.title);
                    Status::Available
                }
            };
        } else {
            println!("\nError: Book with ID {} not found.", id);
        }
    }

    // Calculates and displays collection statistics by genre
    fn display_statistics(&self) {
        let total = self.books.len();
        let checked_out_count = self
            .books
            .iter()
            .filter(|b| matches!(b.status, Status::CheckedOut))
            .count();
        let available_count = total - checked_out_count;

        println!("\n================== LIBRARY STATISTICS ==================");
        println!("Total Books: {}", total);
        println!("Available: {}", available_count);
        println!("Checked Out: {}", checked_out_count);
        println!("=========================================================");
    }

    // Rebuilds the HashMap genre index for fast categorical lookups
    fn rebuild_index(&mut self) {
        self.genre_index.clear();
        for (idx, book) in self.books.iter().enumerate() {
            let key = format!("{:?}", book.genre).to_lowercase();
            self.genre_index.entry(key).or_default().push(idx);
        }
    }

    // Saves the library database to local storage (library.json)
    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        println!("\nLibrary successfully saved to {}.", filename);
        Ok(())
    }

    // Loads the library database from local storage if available
    fn load_from_file(filename: &str) -> Self {
        if Path::new(filename).exists() {
            match fs::read_to_string(filename) {
                Ok(data) => {
                    match serde_json::from_str::<Library>(&data) {
                        Ok(mut lib) => {
                            lib.rebuild_index();
                            println!("Library successfully loaded from {}.", filename);
                            return lib;
                        }
                        Err(e) => println!("Error parsing JSON: {}. Starting fresh.", e),
                    }
                }
                Err(e) => println!("Error reading file: {}. Starting fresh.", e),
            }
        }
        Library::new()
    }
}

// Helper function to read string input from the terminal
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let filename = "library.json";
    let mut library = Library::load_from_file(filename);

    loop {
        println!("\n*** RUST LIBRARY TRACKER MENU ***");
        println!("1. Add New Book");
        println!("2. List All Books");
        println!("3. Toggle Book Status (Check-in / Check-out)");
        println!("4. View Library Statistics");
        println!("5. Save & Exit");

        let choice = get_input("Enter your choice (1-5): ");

        match choice.as_str() {
            "1" => {
                let title = get_input("Enter book title: ");
                let author = get_input("Enter author name: ");
                let genre_input = get_input("Enter genre (Fiction, NonFiction, Science, History, Technology): ");

                if let Some(genre) = Genre::from_str(&genre_input) {
                    library.add_book(title, author, genre);
                } else {
                    println!("Invalid genre entered. Please try again.");
                }
            }
            "2" => {
                library.list_books();
            }
            "3" => {
                library.list_books();
                let id_str = get_input("Enter the ID of the book to toggle status: ");
                if let Ok(id) = id_str.parse::<usize>() {
                    library.toggle_status(id);
                } else {
                    println!("Invalid ID format.");
                }
            }
            "4" => {
                library.display_statistics();
            }
            "5" => {
                if let Err(e) = library.save_to_file(filename) {
                    println!("Failed to save library data: {}", e);
                }
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please enter a number between 1 and 5."),
        }
    }
}