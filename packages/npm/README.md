# @hwai-technology/kora

> Official npm package and CLI launcher for the **Kora Framework**.

## Overview

[Kora](https://github.com/Hwai-technology/kora-framework) is a high-performance, Clean Architecture backend programming language & framework compiling directly to native Rust with zero-cost abstractions, zero garbage collection pauses, and built-in OpenAPI 3.1 & Swagger generation.

## Installation

### Via GitHub Packages (npm registry)

Add `@hwai-technology` to your npm configuration:
```bash
echo "@hwai-technology:registry=https://npm.pkg.github.com" >> ~/.npmrc
```

Install globally:
```bash
npm install -g @hwai-technology/kora
```

Or run directly without installation via `npx`:
```bash
npx @hwai-technology/kora create my-awesome-app
```

## Quickstart

```bash
# Create a new Clean Architecture project
kora create blog-api

# Change directory
cd blog-api

# Start development server on port 7582
kora run

# Open interactive Swagger UI documentation
open http://localhost:7582/docs
```

## Documentation

Full documentation, language specifications, and architecture guides are available at:
[https://github.com/Hwai-technology/kora-framework](https://github.com/Hwai-technology/kora-framework)
