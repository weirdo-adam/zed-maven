# zed-maven

[Maven](https://maven.apache.org) `pom.xml` intelligence for the
[Zed editor](https://zed.dev).

> Original research notes (including the Spring Boot scope that was later
> dropped): see [PLAN.md](./PLAN.md).

## What it does

Attaches a LemMinX-based language server (with the
[lemminx-maven](https://github.com/eclipse-lemminx/lemminx-maven) extension) to
the `XML` language:

- **Schema-aware validation & completion** for `pom.xml` (Maven XSD):
  unknown elements, wrong nesting and invalid values are flagged; element
  completion lists the legal children at the cursor.
- **Dependency completion** inside `<dependency>` blocks:
  - `<groupId>` — group ids from your local `~/.m2` repository
  - `<artifactId>` — artifacts with full `group:artifact:version` labels;
    accepting one auto-fills the surrounding `<groupId>`/`<version>` tags
  - `<version>` — versions available for the artifact
- **Hover** on dependencies for artifact details.

Syntax highlighting, outline and indentation are provided by the official
[`xml`](https://github.com/sweetppro/zed-xml) extension — install it alongside.

## Install (development)

1. Install the `xml` extension from Zed's extension page.
2. Command palette → `zed: install dev extension` → select this directory.
3. Open a Maven project's `pom.xml`.

On first start the extension downloads the server jars automatically from this
repository's [releases](https://github.com/weirdo-adam/zed-maven/releases)
(built by the `release-servers` workflow):

- `lemminx-uber.jar` — LemMinX 0.31.2, patched with
  [`patches/lemminx-resolve-null.patch`](./patches/lemminx-resolve-null.patch)
  so `completionItem/resolve` returns the item instead of `null`
  (Zed cannot deserialize `null`; upstream items without `data` triggered this).
- `lemminx-maven-deps.zip` — lemminx-maven + its dependency jars.

`ZED_LEMMINX_HOME` can point at a hand-managed directory containing
`org.eclipse.lemminx-uber.jar` + `maven-ext/` to bypass the release download.

Requirements: JDK 17+. `java` is resolved via worktree PATH → `ZED_JAVA_HOME`
→ `JAVA_HOME` → common Homebrew/JVM locations.

## Smoke testing

```sh
python3 scripts/smoke.py --java <java-bin> \
  --lemminx-jar <lemminx-uber.jar> --ext-zip <lemminx-maven-deps.zip>
```

Validates: server starts, Maven extension activates, `groupId`/`artifactId`
completion returns items, `completionItem/resolve` returns the item.

## License

MIT
