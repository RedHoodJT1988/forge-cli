# 🔥 Forge
**Forge a new path for Rust web development. One command to rule all your boilerplate.**

Forge is a command-line tool designed to eliminate the boilerplate and setup cost of starting a new full-stack project in Rust. Inspired by frameworks like Django and Rails, `forge` scaffolds a complete, runnable applicaiton with a lightning-fast Axum backend, so you can start building features immediately.

Choose between a modern, reactive frontend with **Dioxus** (all in Rust!) or the elegant simplicity of server-rendered **HTMX**.

## **Core Features**
- **Powerful Rust Backend:** All projects are built on the fast, ergonomic [Axum](https://github.com/tokio-rs/axum) 
- **Flexible Frontend:** Choose your preferred paradigm:
	- **Dioxus:** For building rich, interactive SPAs entirely in Rust (compiled to WASM).
	- **HTMX:** For a traditional, server-rendered HTML approach using the [Maud](https://maud.lambda.xyz/) templating engine.
- **Database Integration:** Out-of-the-box support for multiple databases. The default is a static site with no database.
	- **SQL:** PostgreSQL, MySQL (via `sqlx`)
	- **NoSQL:** MongoDB
	- **BaaS:**: Firebase (via REST API)
- **Ready to Run:** Generated projects include a `.env.example`, pre-configured `Cargo.toml`, and all necdessary connection logic.

## 🚀 **Installation**
Once published, you can install `forge-cli` directly from crates.io:
```bash
cargo install forge-cli
```

### **From Source**
You can also build and install from the source code:
```bash
git clone https://github.com/RedHoodJT1988/forge-cli.git
cd forge
cargo install --path .
```

## **Usage**
The main command is forge new. It takes a project name and optional flags to configure the frontend and database.
```bash
forge new <PROJECT_NAME> [OPTIONS]
```
### **Options**
|Flag|Argument|Description|
|----|--------|-----------|
|`--frontend`| `dioxus` (default), `htmx`| The frontend framework to use.
|`--db`|`postgres`, `mysql`, `mongodb`, `firebase`| The database to configure. If omitted, a static site (no DB) is created.
|`-h`, `--help`| |Print help information.
|--------------------------|

### Examples
1. Create a default static site with Dioxus (no database):
```bash
forge new my_static_site
```
2. Create a static HTMX site (no database):
```bash
forge new my_htmx_blog --frontend htmx
```
3. Create a Dioxus app with PostgreSQL:
```bash
forge new my_store --db postgres
```
4. Create an HTMX dashboard with MongoDB:
```bash
forge new my_dashboard --frontend htmx --db mongodb
```

## **Generated Project Structure**
Running `forge new my_app --db postgres` will generate the following structure:
```bash
my_app/
├── .env.example            # Environment variables (e.g., DATABASE_URL)
├── .gitignore
├── Cargo.toml              # Pre-configured with Axum, Dioxus, SQLx, etc.
├── Dioxus.toml             # Dioxus build configuration
├── index.html              # Entrypoint for the Dioxus WASM app
└── src/
    ├── main.rs             # Axum server entrypoint, routing, and state
    ├── lib.rs                # The root Dioxus application component
    └── db.rs                 # Database connection pool logic and models
```
---
# 💖 **Contributing**
We are thrilled you're interested in contributing to Forge! This project is a community effort, and we welcome help of all kinds, from fixing bugs to adding new features. 

### **How to Contribute**
1. **Find an Issue:** Look through our [GitHub Issues](https://github.com/RedHoodJT1988/forge-cli/issues). Good first issues are a great place to start.
2. **Open an Issue:** If you have a feature idea or find a bug, please open an issue first to discuss it.
3. **Fork and Clone:** Fork the repository and clone it locally.
```bash
git clone https://github.com/RedHoodJT1988/forge-cli.git
cd forge
```
4. **Create a Branch:** Create a new branch for your feature or bugfix.
```bash
git checkout -b feature/my-new-feature
```
5. **Make Your Changes:**
- The core logic is in `src/main.rs` and `src/scaffold/mod.rs`.
- All project blueprints are located in the `templates/` directory.
- To add a feature (e.g., a new database), you would create new template directories (e.g., `templates/redis-htmx` and `templates/redis-dioxus`) and update the `main.rs` file to recognize the new `--db redis` flag.
6. **Test Your Changes:** Build the CLI and test it locally.
```bash
cargo build --release
./target/release/forge-cli new test-project --db YOUR_NEW_DB
cd test-project
cargo run
```
7. **Submit a Pull Request:** Push your branch to your fork and open a Pull Request against the `main` branch of the original repository.

# 🗺️ **Roadmap**
We have big plans for Forge! Here are some of the features we'd love to add:
- [] **Authentication:** Scaffold complete auth logic (registration, login, sessions) for different strategies (e.g., JWT, cookies).
- **[] More Databases:** Add support for `Redis`, `SQLite` (great for simple apps!), and other popular databases.
- **[] Deployment:** Add `Dockerfile` and `fly.toml` generation for easy deployment.
- **[Am] Interactive Mode:** An interactive `forge new` command that walks the user through the setup process.

Your contributions are essential to making this happen!

# 🪪**License**
This project is licensed under the **MIT License**. See the [LICENSE](https://mit-license.org/) file for details.