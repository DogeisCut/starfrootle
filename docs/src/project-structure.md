# Project Structure

> [!NOTE]
> This page is currently a stub and needs more detail.

## Docs
Defines everything you need to know about Starfrootle

## Core Library
Contains things such as the list of opcodes, constants, and other things that could be shared between the assembler, emulator, and other projects.

## Emulator
Contains the actual Starfrootle emulator, including save state management, UI, keyboard/controller mapping, etc.

## Assembler
Rust code for creating Starfrootle rom code, contains a built in library for fonts, sound and music management/playing, and meta/multi-sprite management.

