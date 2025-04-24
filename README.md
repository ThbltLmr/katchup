# KATCHUP
#### Video Demo:  <URL HERE>
#### Description:
Want to watch a show with a friend but they've started without you? New season of your favorite series came out, but you've totally forgotten what happened until now?
Katchup allows you to cherry-pick the season and episode you're about to start, and get a spoiler-free, AI-generated summary of the series up to that point!

#### How to use:
- Search for shows (based on [The Movie DB](https://www.themoviedb.org/))
- Select the season and episode you want to catch up on
- Get a summary of all previous episodes, as well as reminders of key characters, powered by Gemini 2.5

### Project structure
#### API
##### Server
This directory contains the services needed to build the server, parse incoming requests, and respond to them. Incoming requests are currently handled synchronously, without an async runtime.
- thread_pool.rs: Provides a basic thread pool to execute tasks concurrently. It manages worker threads that process jobs received through a channel, including mechanisms for graceful shutdown and basic error/panic handling.
- request_parser.rs: Parses raw incoming HTTP request strings, breaking them down into structured data including the HTTP method, URI (path and query string), headers, and body.
- response_builder.rs: Constructs formatted HTTP response strings. It takes a status code and response data, serializes the data to JSON, and builds the complete response with status line and headers (including CORS).
- router.rs: Maps incoming request URIs to specific actions (like getting show details, cast, search results, or summaries). It coordinates fetching data from external APIs (TMDB, Gemini) via adapters and prepares the response data.

##### Adapters
This directory contains the services used to make external API calls. It uses the reqwest and serde crates for HTTP requests and JSON serialization respectively.
- tmdb_adapter.rs: Handles communication with The Movie Database (TMDB) API to search for shows, get show details (seasons, episode count), and fetch aggregated cast credits (used to get list of cast and characters).
- gemini_adapter.rs: Uses the Google Gemini API to generate summaries of shows up to a specific point and to provide descriptions for lists of characters.
- ollama_adapter.rs (deprecated): Similar to the gemini_adapter.rs file, but using a local Ollama server with llama3.2. Deprecated as performance and quality was much lower than Gemini.
