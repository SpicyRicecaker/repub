#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use epub::doc::EpubDoc;
use std::{fs::File, io::BufReader, sync::Mutex};
use tauri::{command, State};

#[command]
/// Error handling on this func is bad lol
fn change_chapter(state: State<Library>, change: i8) {
  if let Some(book) = state.book.lock().unwrap().as_mut() {
    match change {
      -1 => {
        book.go_prev().unwrap();
      }
      1 => {
        // println!("current book {}", book.set_cur().unwrap());
        book.go_next().unwrap();
      }
      _ => {}
    }
  }
}

#[command]
fn load_chapter(state: State<Library>) -> String {
  if let Some(book) = state.book.lock().unwrap().as_mut() {
    String::from_utf8(book.get_current_with_epub_uris().unwrap()).unwrap()
  } else {
    "error".to_string()
  }
}

#[command]
fn init_book(state: State<Library>) {
  use epub::doc::EpubDoc;
  let mut doc = EpubDoc::new("C:\\Users\\spicy\\git\\epub\\static\\86.epub").unwrap();

  doc.set_current_page(10).unwrap();

  *state.book.lock().unwrap() = Some(doc);
}

struct Library {
  book: Mutex<Option<EpubDoc<BufReader<File>>>>,
  test: Mutex<u32>,
}

fn main() {
  tauri::Builder::default()
    .manage(Library {
      book: Mutex::new(None),
      test: Mutex::new(0),
    })
    .invoke_handler(tauri::generate_handler![
      change_chapter,
      load_chapter,
      init_book
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
