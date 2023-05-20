# Contributing to SQLx-ts

All sort of contributions are welcome and there are no complicated rules with it.
We appreciate:

- New features
- Bug fixes
- Suggestions
- Ideas

## Issues

Feel free to submit issues, ideas, suggestions and enhancement requests.

## Contributing

Please refer to each project's style guidelines and guidelines for submitting patches and additions.
In general, we follow the "fork-and-pull" Git workflow.

1.  **Fork** the repo on GitHub
2.  **Clone** the project to your own machine
3.  **Commit** changes to your own branch
4.  **Push** your work back up to your fork
5.  Submit a **Pull request** so that we can review your changes

NOTE: Be sure to merge the latest from "upstream" before making a pull request!

All core contributors have a permission to simply create a sub-branch of main for contribution.


## Development Environments

| type  | version                            |
|-------|------------------------------------|
| rustc | stable - check rust-toolchain.toml |
| OS    | Linux, Windows and Mac             |


#### Setting up development environment

[Please use rust-toolchain.toml](https://rust-lang.github.io/rustup/concepts/toolchains.html) to ensure that we are 
all using the same Rust tooling for the development. 

As the sqlx-ts's core is essentially a CLI application that needs a database to connect.
You will need a running database instance.

1. Spin up DBs using docker-compose.yaml

`docker-compose -d up`

Should be spinning up postgres mainly, but we will be dealing with other types of DBs as we go in the future.

2. Set up environment variables for DB connection.

The easiest way to set up DB credentials to connect to the primary database is by providing a `.env` in the
project root

```dotenv
DB_HOST=127.0.0.1
DB_PORT=54321
DB_USER=postgres
DB_PASS=postgres
```

You can also set these up by CLI arguments. Please run `--help` on `/target/debug/sqlx-ts` binary to learn more.

At this point, you should be all set up to start contributing to sqlx-ts! :rocket:

## Testing



## Releasing

We release by creating a new Github release and a git tag. Each tag is automatically picked up by Github Action 
and continuously deliver new versions to the users.

## Copyright and Licensing

sqlx-ts is an open source project licensed under the MIT license.

sqlx-ts does not require you to assign the copyright of your contributions, you retain the copyright.
sqlx-ts does require that you make your contributions available under the MIT license in order to be
included in the main repo.

If appropriate, include the MIT license summary at the top of each file along with the copyright info.
If you are adding a new file that you wrote, include your name in the copyright notice in the license
summary at the top of the file.

## License Summary

You can copy and paste the MIT license summary from below.

```
MIT License

Copyright (c) 2022 Jason Shin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```
