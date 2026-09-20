# zed-spring-boot

Maven & Spring Boot intelligence for the [Zed editor](https://zed.dev).

> Roadmap and research notes: see [PLAN.md](./PLAN.md).

## Relationship to the `xml` extension

Syntax highlighting, outline and indentation for XML are provided by the
official [`xml`](https://github.com/sweetppro/zed-xml) extension — this
extension **does not duplicate** that. Instead, `spring-boot` attaches
language servers on top of the `XML` language:

| Capability | Extension | Status |
|---|---|---|
| XML syntax / outline / indent | `xml` (install separately) | ✅ available |
| Schema-aware completion & validation for `pom.xml` (LemMinX) | **spring-boot** | 🚧 M1 |
| Dependency `groupId/artifactId/version` completion (lemminx-maven) | **spring-boot** | 🚧 M2 |
| `application.properties`/`.yml` intelligence (spring-boot-language-server) | **spring-boot** | 🚧 M3 |
| Snippets & `@SpringBootApplication` runnables | **spring-boot** | 🚧 M4 |

`contrib/upstream-zed-xml/` holds extra/validated queries (e.g.
`brackets.scm`) intended to be contributed upstream.

## Install (development)

1. Install the `xml` extension from Zed's extension page.
2. Command palette → `zed: install dev extension` → select this directory.
3. Open [`testdata/pom.xml`](./testdata/pom.xml).

> Note: the dev-extension install registers the directory path, so after this
> rename you'll need to re-install the dev extension from the new location
> (`~/Developer/github/zed-spring-boot`).

## License

MIT
