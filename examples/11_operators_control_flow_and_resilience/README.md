# Example 11: Operators, Control Flow & Resilient Error Handling

> **Comprehensive Showcase of Language Modernization & Resilience**  
> This example demonstrates Kora's modern operator suite (`??`, `+=`, `-=`, `*=`, `/=`, `..`), structured control flow (`for..in`, `break`, `continue`, `const`), and automated `try / catch / finally` resilience.

---

## ⚡ 1. Modern Operators Suite

| Operator | Syntax | Description | Example |
|:---|:---:|:---|:---|
| **Nullish Coalescing** | `??` | Safely evaluates optional/nullable values with a fallback, avoiding guessing | `let timeout = request.timeout ?? 30` |
| **Compound Addition** | `+=` | Adds and reassigns in-place | `processedCount += 1` |
| **Compound Subtraction** | `-=` | Subtracts and reassigns in-place | `remainingQuota -= 1` |
| **Compound Multiplication** | `*=` | Multiplies and reassigns in-place | `delay *= attempt` |
| **Compound Division** | `/=` | Divides and reassigns in-place | `balance /= 2` |
| **Range Construction** | `..` | Creates a typed range or sequence of values | `0..10` |
| **Pipe Operator** | `\|>` | Passes an expression into a callable pipeline | `data \|> transformer` |

---

## 🔄 2. Structured Control Flow

### A. For-In Loops
Iterate effortlessly over arrays, collections, and ranges without manual index tracking:
```kora
for item in request.payloads {
    if item == "SKIP" {
        continue; // Skips current iteration
    }
    if totalCount >= 100 {
        break; // Exits loop early
    }
    totalCount += 1;
}
```

### B. Compile-Time Constants (`const`)
Declare immutable values verified at compile-time:
```kora
const DEFAULT_TIMEOUT_SECONDS: Int = 30;
const MAX_RETRIES: Int = 5;
```

---

## 🛡️ 3. Automatic Error Handling (`try / catch / finally`)

Catch and recover from unexpected runtime exceptions or service failures gracefully:
```kora
try {
    if request.payloads.isEmpty() {
        throw "Cannot process empty batch payloads";
    }
    // Execution steps...
} catch (err) {
    // Graceful error logging & fallback response
    lastError = Some(err);
} finally {
    // Guaranteed cleanup block
    recordTelemetryMetrics();
}
```

### Automatic Resilience by Default
When scaffolding features with `kora g feature <name>`, all controller endpoints and service implementations are automatically generated with built-in `try / catch` error boundaries by default to prevent unexpected panics and guarantee 99.999% server availability.

---

## 🛠️ 4. Toolchain CLI Commands

| Command | Purpose |
|:---|:---|
| `kora test [path]` | Runs the automated test runner across `.kora` files or suites with instant pass/fail metrics |
| `kora clean` | Purges `.kora-build/` artifacts, binary caches, and temporary compile files |
| `kora doctor` | Runs a complete health check verifying Kora, Rust compiler, Cargo, and DB drivers |

---

## 🚀 Running This Example

### Type Check & Verify:
```bash
cargo run -p kora-cli -- check examples/11_operators_control_flow_and_resilience/main.kora
```

### Run Automated Tests:
```bash
cargo run -p kora-cli -- test examples/11_operators_control_flow_and_resilience/main.kora
```

### Run Toolchain Diagnostics:
```bash
cargo run -p kora-cli -- doctor
```
