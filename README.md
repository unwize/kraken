# kraken
A simple toy transactions engine that processes the payments crediting and debiting accounts.

## Nature of Documentation, Descriptions, and Explanations
For some, Tokio and Polars may be second-nature--but I've not used them much, and never in a performance-centric environment. The text I've written here serves two purposes: to keep myself oriented regarding important info as I learn, and to inform 3rd-party observers about my intentions. As such, code utilizing crates or methods I'm familiar with may have significantly less commenting, explaining, etc. Code built using unfamiliar technologies may have robust (and occasionally verbose) comments to maintain focus and internal clarity.

## Tooling
- RustRover (IDE)

## Dependencies
The top-level dependencies are as follows:

- [Miette](https://github.com/zkat/miette): Diagnostics and Error Handling
  - [Trace](https://github.com/tokio-rs/tracing): Proper tracing and diagnostics (Stretch goal)
- [csv-async](https://docs.rs/csv-async/latest/csv_async/): Async CSV reader that supports streaming
- [Tokio](https://docs.rs/tokio/latest/tokio/): Async runtime
- [Polars](https://github.com/pola-rs/polars): Data query engine (Rust version of Pandas, kind-of)
  - [Columnar data format](https://arrow.apache.org/docs/format/Columnar.html)
  - [Comprehensive list of data format specifications](https://arrow.apache.org/docs/format/Columnar.html)
## How it works

There's a few assumptions being made here:
- Transactions are not read in chronological order, but are assigned IDs in chronological order. So, a transaction with ID `n` is created before a transaction with ID `n + 1`.
- Transactions may not have ID `< 0`.
- The flow of "destructive" operations is fixed and may not occur out of order. The following would result in `tx` `1` being disputed and **not** resolved, as the resolve was dropped.
  - If each transaction had its own ID, and didn't overload the meaning of the `tx` column, this could be fixed by issuing IDs chronologically. For example, if the `resolve` tx had an ID of 6 and the `dispute` tx had an ID of 5, the ordering would matter less and you could logically assert their correctness.
  - As it stands, one has no way of knowing, if this scenario is a simple "out of order" issue or two very poorly-timed mistakes. 
    - One could implement a "forgiveness" mechanism that allows a required predecessor to appear `n` transactions late and still be valid. 

| type    | client | tx | amount |
|---------|--------|----|--------|
| deposit | 2      | 1  | 2.0    |
| resolve | 2      | 1  | _      |
| dispute | 2      | 1  | _      |



For a given CSV of unordered transactions, sanitize them and insert them into a dataframe. For a given transaction, its **primary key** is its **transaction ID**.

To compute the current state of an account, "render" it. Rendering an account extracts all relevant transactions from the dataframe, sorts them chronologically, then steps through each transaction. Once rendered, the now-sorted transactions and state of the account are cached in an independent dataframe.
  - A potentially-unsafe optimization, storage-wise, would be to remove the original statements in the main dataframe.
  - Disputes, resolutions, and chargebacks must be 