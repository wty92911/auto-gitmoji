# Init Spec

## Overview
I want to create a tool with `Rust` that will add a gitmoji to the commit message.
For example: when I type `amoji "add a new feature"`, it will run the script below:
```
git commit -m ":sparkles: add a new feature"
```
And print the final commit message to the console with the exactly gitmoji not the emoji code.
In short, the core functionality is to find the nearest gitmoji to the commit message.

## Core Functionality
There are many methods to match the commit message to a gitmoji.
1. Keyword Matching (Simple)
2. Semantic Matching (Model)
3. LLM Matching (Api)

So you should extract the common interface from these methods for future extension.

So help me to design the whole project as a spec document for `Cursor` to implement.



