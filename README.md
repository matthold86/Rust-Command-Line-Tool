# Rust Command Line Tool

[![Format](https://github.com/nogibjj/mjh140-cmdline-tool/actions/workflows/format.yml/badge.svg)](https://github.com/nogibjj/mjh140-cmdline-tool/actions/workflows/format.yml)  [![Lint](https://github.com/nogibjj/mjh140-cmdline-tool/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/mjh140-cmdline-tool/actions/workflows/lint.yml)  [![Tests](https://github.com/nogibjj/mjh140-cmdline-tool/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/mjh140-cmdline-tool/actions/workflows/tests.yml)

## Summary

The project is an introduction to Rust packaging and Rust command line tools. The objective of this tool is to randomly generate a list of fruits to be put into a fruit salad. The user inputs a number of fruits and the list of fruits is printed in the command line. The user is prompted to evaluate the list determine if they like the fruit salad. If they do, a user input "yes" will exit the script. If they do not like the salad, a user input "no" will generate a new random fruit salad. The script is looped until the user is satisfied.

To learn more about the random fruit generator visit this repository: [cli-salad](https://github.com/nogibjj/rust-data-engineering/blob/main/cli-salad/src/lib.rs)


## Structure
```text
mjh140-cmdline-tool/
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
├── Makefile
├── README.md
├── requirements.txt
└── src
    ├── lib.rs
    └── main.rs
```

## Results
This script is configured to run on Github Codespaces or directly on VS Code. If running locally, ensure Rust is installed on your local system.

The script takes one parameter: number of fruit to include in the fruit salad. For this example we will use `5`. In the terminal, run the following command:

`cargo run -- -n 5`

![image](https://github.com/nogibjj/mjh140-cmdline-tool/assets/114833075/8d2b2107-015b-464e-b747-9b74c6699677)

Within the terminal, the list of fruit will be printed and the user will be given the option to regenerate a new list or accept the provided list. To regenerate the list, type `no`. To accept the list type `yes`

![image](https://github.com/nogibjj/mjh140-cmdline-tool/assets/114833075/28c5564f-c29f-4794-9050-b8e5686f31fa)

The fruit salad will regenerate as many times as needed until the list is accepted.

![image](https://github.com/nogibjj/mjh140-cmdline-tool/assets/114833075/a7457112-382a-44ab-a080-a4035319c80c)



