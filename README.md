# zed-spring-suite

Maven & Spring Boot intelligence for the [Zed editor](https://zed.dev).

> Roadmap and research notes: see [PLAN.md](./PLAN.md).

## Relationship to the `xml` extension

Syntax highlighting, outline and indentation for XML are provided by the
official [`xml`](https://github.com/sweetppro/zed-xml) extension — this
extension **does not duplicate** that. Instead, `spring-suite` attaches
language servers on top of the `XML` language:

| Capability | Extension | Status |
|---|---|---|
| XML syntax / outline / indent | `xml` (install separately) | ✅ available |
| Schema-aware completion & validation for `pom.xml` (LemMinX) | **spring-suite** | 🚧 M1 |
| Dependency `groupId/artifactId/version` completion (lemminx-maven) | **spring-suite** | 🚧 M2 |
| `application.properties`/`.yml` intelligence (spring-boot-language-server) | **spring-suite** | 🚧 M3 |
| Snippets & `@SpringBootApplication` runnables | **spring-suite** | 🚧 M4 |

`contrib/upstream-zed-xml/` holds extra/validated queries (e.g.
`brackets.scm`) intended to be contributed upstream.

## Install (development)

1. Install the `xml` extension from Zed's extension page.
2. Command palette → `zed: install dev extension` → select this directory.
3. Open [`testdata/pom.xml`](./testdata/pom.xml).

## License

MIT
