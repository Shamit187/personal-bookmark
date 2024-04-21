# Personal Bookmark Project

## Description

The Personal Bookmark Project is a web application designed to visualize my book collection and track my progress with each book. Notes from every chapter can also be added to the application using API (to be built). It's built using Actix-Web, HTMX, and TailwindCSS, providing a sleek and responsive user interface. (Database to be added)

## Technologies Used

### Actix-Web

Actix-Web is a powerful, high-performance web framework for Rust, providing robust features for building web applications. With Actix-Web, you can easily handle HTTP requests, manage state, and create efficient web servers.

### HTMX

HTMX is a library that allows you to access AJAX, WebSockets, and server-sent events directly in HTML, making it easy to add dynamic functionality to your web pages without having to write JavaScript. With HTMX, you can enhance user interactions and update content dynamically, leading to a smoother user experience.

### TailwindCSS

TailwindCSS is a utility-first CSS framework that enables you to quickly build custom designs without having to write CSS from scratch. With TailwindCSS, you can create responsive layouts, apply stylish designs, and maintain consistency across your web application effortlessly.

## Features

- **Book Visualization**: View all my books in an organized manner, including their titles, authors, and current progress.
- **Progress Tracking**: Track my progress with each book, whether it's through page numbers, chapters, or percentages completed.
- **Note Taking**: Add notes for each chapter of a book, allowing me to keep track of important details and insights.
- **API Integration**: Integrate an API to manage the database, allowing me to add, update, and delete books and notes.
- **Design Philosophy**: Implement a sleek and responsive user interface using HTMX and TailwindCSS, providing a smooth and enjoyable user experience without the need for heavy JavaScript.

## Roadmap

- [X] **Database Integration**: Integrate a database to store book and note data, allowing for persistent storage and retrieval.
- [x] **Add more Type of Content Support in Note**: items that I wish to add:
  - [X] Title
  - [X] Text
  - [x] Image/ Figures
  - [x] Table
  - [x] List
  - [x] Highlighted Text
  - [x] Author
  - [x] Date
  - [x] Quote
  - [x] Definition
- [X] **Figuring out using Latex in HTMX**: Implement a way to use Latex in HTMX to display mathematical equations and formulas in the notes.
- [ ] **Progress Visualization**: Visualize my progress with each book using charts and graphs, providing a clear overview of my reading habits and patterns.
- [ ] **Forum Integration**: Integrate a forum to discuss books and share insights with other users, creating a community around reading and learning.

## Pages

* [x] "/"
  * [X] basic stuff
  * [x] background image
* [x] "/explore"
  * [X] basic menu
  * [x] click to show every book in that genre
  * [x] write api for that
* [X] "/test_note"
* [x] "/notes/{book_id}/{note_id}"
  * [x] basic stuff
  * [x] change color mode logo
  * [ ] if chapter == 0 : show thoughts on book
* [ ] "/book_search/{search_key}"

## Host

[https://personal-bookmark.onrender.com/](https://personal-bookmark.onrender.com/)

"Do wait a few minutes, renders take time to load"

## How to run

add .env file with the following content:

```env
PORT=<>
MONGOURI=<>
```

Run the following command:

```bash
# for production version
cargo run --release
 
# for dev version
cargo run
```

## How to add new page

1. create a new folder in the 'static' folder
2. create a new html file in the new folder named 'index.html'

## How to add new api

Currently, all in one file, 'src/api.rs'(to be changed)

## Dependencies

- actix-web = "4"
  - For almost everything
- actix-files = "0.6.5"
  - For Static File server
  - For Favicon
- dotenv = "0.15.0"
  - Loading environment
- serde = "1.0"
  - Serialization Deserialization
  - Currently Not that needed
- serde_json = "1.0"
  - Same use as serde
- mongodb = "2.8.1"
  - Main Database
- futures = "0.3"
  - Dependency for mongo db query

## Learning Process

* [Actix](LearningActix.md)
* [HTMX](LearningHTMX.md)
* [Tailwind CSS](LearningTailwind.md)
