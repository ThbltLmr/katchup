# KATCHUP
#### Video Demo:  <URL HERE>
#### Description:
Want to watch a show with a friend but they've started without you? New season of your favorite series came out, but you've totally forgotten what happened until now?
Katchup allows you to cherry-pick the season and episode you're about to start, and get a spoiler-free, AI-generated summary of the series up to that point!

#### How to use:
- Search for shows (based on [The Movie DB](https://www.themoviedb.org/))
- Select the season and episode you want to catch up on
- Get a summary of all previous episodes, as well as reminders of key characters, powered by Gemini 2.5

## Project structure

![image](https://github.com/user-attachments/assets/c0088906-2d47-4c01-a569-761c7e5feef6)

### API - Implementation of thread-pool server and adapters for external services in Rust
#### Server
This directory contains the services needed to build the server, parse incoming requests, and respond to them. Incoming requests are currently handled synchronously, without an async runtime.
- thread_pool.rs: Provides a basic thread pool to execute tasks concurrently. It manages worker threads that process jobs received through a channel, including mechanisms for graceful shutdown and basic error/panic handling.
- request_parser.rs: Parses raw incoming HTTP request strings, breaking them down into structured data including the HTTP method, URI (path and query string), headers, and body.
- response_builder.rs: Constructs formatted HTTP response strings. It takes a status code and response data, serializes the data to JSON, and builds the complete response with status line and headers (including CORS).
- router.rs: Maps incoming request URIs to specific actions (like getting show details, cast, search results, or summaries). It coordinates fetching data from external APIs (TMDB, Gemini) via adapters and prepares the response data.

#### Adapters
This directory contains the services used to make external API calls. It uses the reqwest and serde crates for HTTP requests and JSON serialization respectively.
- tmdb_adapter.rs: Handles communication with The Movie Database (TMDB) API to search for shows, get show details (seasons, episode count), get episode overviews (used as a starting point for the LLM), and fetch aggregated cast credits (used to get list of cast and characters).
- gemini_adapter.rs: Uses the Google Gemini API to generate summaries of shows up to a specific point and to provide descriptions for lists of characters.
- ollama_adapter.rs (deprecated): Similar to the gemini_adapter.rs file, but using a local Ollama server with llama3.2. Deprecated as performance and quality was much lower than Gemini.

#### Entry point - main.rs
When the server receives a request:
- the request is assigned to one of the threads of the thread pool.
- the request is parsed and the route and parameters are extracted.
- if the request matches an implemented route, it is handled, otherwise a 404 error is returned.
- depending on the request, data is retrieved from the TMDB API, the Gemini API or both.
- the retrieved data is formatted and a 200 OK response is returned with the formatted data.

### WEBAPP - React app built with Vite, Tanstack Query, and ShadCN
#### Assets
This directory contains the logo of the app (generated with ChatGPT) in SVG format

#### Hooks
This directory contains hooks using Tanstack Query to make HTTP requests to the server.
- useGetCast.ts: fetches the cast list, along with the character they play and an AI-generated description of that character.
- useGetShow.ts: fetches information about a given show (number of seasons, episodes...).
- useGetSummary.ts: fetches AI generated summary of a show up to a given season and episode.
- useSearchShows.ts: fetches a list of shows matching a search query.
- useGetTmdbImage.ts: build image URL of a TMDB profile image.

#### Components
- ui directory: collection of ShadCN components used in the app.
- providers: theme provider from ShadCN.
- SearchBar.tsx: search bar component, using useSearchShows hook.
- SeasonDropdown.tsx and EpisodeDropdown.tsx: dropdown menus to select a season and episode once a show has been selected (based on data fetched from useGetShow hook).
- CastList.tsx: carrousel component with cards for the show characters, using the data from the useGetCast hook.
- ShowSummary.tsx: summary of the show (using data from useGetSummary hook).

#### Entry point - App.tsx
The state for the selected show, season and episode are managed at a global scope in App.tsx.
Components are rendered conditionally as the user makes their selection.


### OLLAMA (deprecated) - Dockerfile and shell script to deploy ollama server
This directory contains a Dockerfile and an associated shell script to start an ollama server with llama3.2. It is not needed since the API now uses gemini_provider.rs.
The docker-compose file has been edited to remove the ollama image as well.
