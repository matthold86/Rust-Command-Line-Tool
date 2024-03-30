# College Basketball Database - Rust Command Line Tool

[![Format](https://github.com/nogibjj/mjh140-rusqliteCLI/actions/workflows/format.yml/badge.svg)](https://github.com/nogibjj/mjh140-rusqliteCLI/actions/workflows/format.yml)  [![Lint](https://github.com/nogibjj/mjh140-rusqliteCLI/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/mjh140-rusqliteCLI/actions/workflows/lint.yml)   [![Tests](https://github.com/nogibjj/mjh140-rusqliteCLI/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/mjh140-rusqliteCLI/actions/workflows/tests.yml)

## Summary

This project creates a command line tool to interact with a rusqlite database for college basketball statistics.  The data included in this database includes every D1 college basketball team from 2002 to 2017. Data was scraped from the KenPom Advanced Basketball Analytics website.

The objective of this tool is to allow a user to interact with the college basketball statistics database using the command line interface. The user will pass SQL queries into the terminal and the commands will be performed on the rusqlite database. The data is re-loaded from a csv file after every run, so database manipulation while the program is running is allowed and encouraged.

Click [here](https://youtu.be/GGBqJJJ7MFY) for a video walkthrough of the project.


## Structure
```text
mjh140-rusqliteCLI/
├── Cargo.lock
├── Cargo.toml
├── .devcontainer
│   ├── Dockerfile
│   └── devcontainer.json
├── .github/workflows
│   ├── format.yml
│   ├── install.yml
│   └── tests.yml
├── .gitignore
├── kenpom.csv
├── kenpom.db
├── Makefile
├── README.md
├── requirements.txt
└── src
    ├── lib.rs
    └── main.rs
```

## Results

When the code is first executed, data is loaded from a csv file into the kenpom database. Below is a picture of the initial rusqlite database:

![image](https://github.com/nogibjj/mjh-miniproject8/assets/114833075/c21591e7-5f57-461b-8e98-90dd430c671d)

After the database connection has been made and the data has been loaded, the terminal becomes a SQL query interface to interact with the database. The terminal will show the following prompt to indicate that the CLI is operational:

![image](https://github.com/nogibjj/mjh-miniproject8/assets/114833075/d8d36315-0859-4461-8818-117b13614a2c)

The prompt says that the user has 10 queries before the script will exit, but the user will be prompted to continue if the quota of 10 queries is reached. Lets walk through some SQL commands and see how they appear on the terminal. Below you'll find examples of SQL commands executed on the database as well as a picture of the returned output from the terminal.

___
`SELECT year, team, seed FROM kenpom_stats WHERE team = 'Michigan';`

![image](https://github.com/nogibjj/mjh-miniproject8/assets/114833075/d19c3d93-30d4-4b3f-911b-6543bd9c9b63)
___
`UPDATE kenpom_stats SET seed = NULL WHERE seed = "";`

`SELECT year, team, seed FROM kenpom_stats WHERE team = 'Michigan';`

![image](https://github.com/nogibjj/mjh-miniproject8/assets/114833075/a9332479-9830-43b2-9cf0-a3e59bd2b3a2)
___
`INSERT INTO kenpom_stats (year, team, wins, losses) VALUES (2018, 'Villinova', 36, 4);`

`SELECT year, team, wins, losses, conference FROM kenpom_stats WHERE year = 2018;`

![image](https://github.com/nogibjj/mjh-miniproject8/assets/114833075/ed9f93a8-23ea-4fff-9d54-5b5cf68db172)
___
`DELETE FROM kenpom_stats WHERE year = 2018;`

`SELECT year, team, wins, losses, conference FROM kenpom_stats WHERE year = 2018;`

![image](https://github.com/nogibjj/mjh-miniproject8/assets/114833075/ed5230f4-9928-46b6-9f8c-eba8610da1a0)
___
`Exit`

![image](https://github.com/nogibjj/mjh-miniproject8/assets/114833075/dff8a1cd-9bc6-479c-af68-9ddd74ab56a6)
___

