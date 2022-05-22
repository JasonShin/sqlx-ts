# Troubleshooting

## MacOS

#### Running the binary simply returns `[1]    <PID> killed     ./sqlx-ts --help`

This is because sqlx-ts haven't yet solved the developer license issue described https://users.
rust-lang.org/t/distributing-cli-apps-on-macos/70223/13 for MacOS specifically.

To fix this, you will need to goto `System Preferences -> Security & Privacy`

<img src="https://i.imgur.com/nGjqlgI.png" width="350px">

You will see `sqlx-ts` binary blocked. Please manually enable it and you can start using sqlx-ts
locally.

<br />
