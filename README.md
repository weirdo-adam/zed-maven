# zed-maven

[Maven](https://maven.apache.org) `pom.xml` intelligence for the
[Zed editor](https://zed.dev).

## Features

Runs [LemMinX](https://github.com/eclipse-lemminx/lemminx) with
[lemminx-maven](https://github.com/eclipse-lemminx/lemminx-maven) on `pom.xml`:

- Schema-aware validation & completion (Maven XSD)
- Dependency completion: `<groupId>` / `<artifactId>` (with
  `group:artifact:version` labels, auto-fills sibling tags) / `<version>`
- Hover details on dependencies

Syntax highlighting comes from the official `xml` extension — install it too.

## Install (development)

1. Install the `xml` extension from Zed's extension page.
2. `zed: install dev extension` → select this directory.
3. Open a Maven project's `pom.xml`. Server jars (~19 MB) download
   automatically from [releases](https://github.com/weirdo-adam/zed-maven/releases)
   on first start.

Requires JDK 17+. `java` resolution: worktree PATH → `ZED_JAVA_HOME` →
`JAVA_HOME` → common Homebrew/JVM paths. `ZED_LEMMINX_HOME` can point at a
hand-managed dir (`org.eclipse.lemminx-uber.jar` + `maven-ext/`) to bypass
release downloads.

## Development

- `patches/` — LemMinX is built from the 0.31.2 tag with
  `lemminx-resolve-null.patch` (`completionItem/resolve` must not return
  `null`; Zed cannot deserialize it, and the fix unlocks
  `additionalTextEdits` for dependency completion).
- `.github/workflows/release-servers.yml` — packages and publishes the jars.
- `scripts/smoke.py` — LSP smoke test over the packaged jars:

  ```sh
  python3 scripts/smoke.py --java <java> \
    --lemminx-jar <lemminx-uber.jar> --ext-zip <lemminx-maven-deps.zip>
  ```

Research notes and original roadmap: [docs/PLAN.md](./docs/PLAN.md).

## License

MIT
