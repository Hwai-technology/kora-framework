# Compile-Time Dependency Injection & Ownership in Kora

Unlike languages that rely on dynamic runtime reflection or heavy container decorators (such as Spring or NestJS), Kora provides **Compile-Time Inversion of Control (IoC)**.

```
Zero Runtime Overhead · No Reflection · Cycle Detection at Compile-Time
```

---

## 1. How It Works

1. **Explicit Parameter Declaration**: Classes declare their dependencies in their constructor (`init`):
   ```kora
   class UsersController {
       service: UsersService,
       logger: LoggerService,

       init(service: UsersService, logger: LoggerService) {
           this.service = service
           this.logger = logger
       }
   }
   ```
2. **Compiler Dependency Graph**: During the HIR lowering and ownership phase (`kora_ownership`), the Kora compiler builds a directed acyclic graph (DAG) of all classes and their parameters.
3. **Topological Sort**: Dependencies are topologically ordered and instantiated from the leaf nodes to the root.
4. **Ownership Plan Emission**: The compiler emits zero-cost native code where dependencies are wrapped in thread-safe reference counting (`Arc<T>`) or passed by value according to borrow analysis.

---

## 2. Circular Dependency Detection

If two services depend on each other:

```kora
class ServiceA {
    b: ServiceB,
    init(b: ServiceB) { this.b = b }
}

class ServiceB {
    a: ServiceA,
    init(a: ServiceA) { this.a = a }
}
```

The Kora compiler halts compilation immediately with `error[E3001]`:

```text
error[E3001]: circular dependency detected in service constructor graph:
  --> ServiceA -> ServiceB -> ServiceA
  💡 Break the cycle by extracting shared state into a repository or separate domain service.
```

---

## 3. Benefits of Compile-Time DI

- **Zero Startup Lag**: Applications boot instantaneously (0.01s) without scanning classpaths or creating dynamic proxy objects.
- **Fail-Fast Safety**: Missing dependencies, type mismatches, or cycles are caught in your editor and during `kora check`, before running in production.
- **100% Mockable**: In unit tests, simply pass mock implementations conforming to the interface without needing complex mocking frameworks.

