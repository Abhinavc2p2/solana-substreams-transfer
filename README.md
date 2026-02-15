# Solana Transfer Extraction – Substreams Assignment

## Overview
This project extracts SOL transfer transactions from Solana blockchain blocks:
385870151 to 385870156.

## Tech Used
- Rust
- Substreams
- Solana

## How to Run
substreams build

substreams run \
-e mainnet.sol.streamingfast.io:443 \
my-project-v0.1.0.spkg \
map_my_data \
-s 385870151 \
-t 385870156 \
-o jsonl

## Output Format
{"from": "...", "to": "...", "amount": 12345}

## Screenshots
See /asset folder
