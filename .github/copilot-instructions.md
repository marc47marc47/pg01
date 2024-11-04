# GitHub Copilot Instructions

## Goal
To write Rust code that parses TCP packets, specifically focusing on PostgreSQL application layer packets, and extracts SQL commands from them.

## Steps

1. **Set up the Rust project**:
    - Create a new Rust project using `cargo new project_name`.
    - Add necessary dependencies in `Cargo.toml`, such as `pnet` for packet parsing.

2. **Write code to capture TCP packets**:
    - Use the `pnet` crate to capture TCP packets.
    - Filter packets to capture only those related to PostgreSQL.

3. **Parse PostgreSQL packets**:
    - Implement logic to parse the PostgreSQL protocol.
    - Extract the application layer data from the TCP packets.

4. **Extract SQL commands**:
    - Analyze the PostgreSQL packet data to identify and extract SQL commands.
    - Handle different types of SQL commands and their structures.

## Example Code Snippets

### Setting up dependencies in `Cargo.toml`


# 專案背景
我們的專案使用 Rust 撰寫，需要處理 TCP 層的封包，並解析其中的 PostgreSQL 應用層封包內容。特別是，我們要解析 SQL 指令，並且希望獲得符合 PostgreSQL 通訊協議的具體實作。

# 編碼風格與需求
1. 我們使用 Rust 開發時，遵循標準的 Rust 錯誤處理模式，例如 `Result<T, E>`，並使用 `?` 運算子進行錯誤傳播。
2. 解析 TCP 封包時，我們希望使用 crate `pnet` 進行封包捕獲與解析。
3. 對於 PostgreSQL 封包解析，我們使用 crate `postgres-protocol` 來處理應用層封包。
4. 在解析封包時，請注意 PostgreSQL 的通訊協議，並解析出 SQL 查詢語句。
5. 我們需要在解析出 SQL 指令後，能夠進行基本的查詢語法分析，確保 SQL 語法的正確性。
6. 程式碼必須具備高度模組化，確保 TCP 解析、PostgreSQL 應用層解析和 SQL 指令處理等部分可以單獨測試。

# 程式碼產生具體指引
- 捕獲並解析 TCP 封包的程式碼，應使用 `pnet::datalink` 進行封包擷取。
- 對於 PostgreSQL 應用層封包，使用 `postgres-protocol` 來進行解析，並從封包中提取出 SQL 指令。
- 我們需要能夠處理 PostgreSQL 通訊中的查詢命令，解析其中的 SQL 語法。
- 請遵守 PostgreSQL 通訊協議的標準，並確保封包處理時正確處理 PostgreSQL 特有的協議訊息（例如：`Query`、`Parse`、`Bind`、`Execute`）。
- 請撰寫單元測試，特別針對 TCP 層的封包捕捉、PostgreSQL 封包解析、SQL 指令提取等模組。
- 在解析 SQL 指令時，請確保結果能夠被打印出來，供後續分析使用。

# 額外注意事項
1. 我們使用 Cargo 進行建置與相依性管理，請確保生成的程式碼能夠無誤地透過 `cargo build` 編譯並執行。
2. 若有可能，請根據程式碼加上合適的註解，解釋其中的關鍵步驟，尤其是對於 TCP 和 PostgreSQL 封包的解析部分。
3. 在解析 SQL 時，優先解析查詢語句，並忽略其他 PostgreSQL 指令。
4. 所有的copilot chat都用繁體中文回答