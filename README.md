# zed-maven

English | [中文](README.zh-CN.md)

[Maven](https://maven.apache.org) `pom.xml` intelligence for the
[Zed editor](https://zed.dev).

## Feature matrix

Runs [LemMinX](https://github.com/eclipse-lemminx/lemminx) with
[lemminx-maven](https://github.com/eclipse-lemminx/lemminx-maven) on `pom.xml`:

| Feature | Status | Notes |
|---|---|---|
| Schema validation (Maven XSD) | ✅ | illegal elements/nesting reported as diagnostics |
| Element & attribute completion | ✅ | schema-aware |
| Dependency `<groupId>` completion | ✅ | from local `~/.m2` repository |
| Dependency `<artifactId>` completion | ✅ | `group:artifact:version` labels; auto-fills sibling tags |
| Dependency `<version>` completion | ✅ | versions available for the artifact |
| `<scope>` / execution `<phase>` completion | ✅ | enum values |
| Maven property `${...}` completion | ✅ | properties from the effective model |
| Hover | ✅ | element docs (XSD) + artifact details |
| Code actions | ✅ | extract/inline `${property}`, remove managed version, remove id part |
| Rename Maven property | ✅ | |
| Go-to-definition for properties | ✅ | |
| XML formatting | ✅ | LemMinX built-in |
| Maven Central remote search | ⚠️ | optional, network-dependent |
| Dependency tree | ✅ | via a Zed task, see below |

Syntax highlighting is provided by the official `xml` extension; install it alongside.

### Dependency tree (task)

Add to your project's `.zed/tasks.json` and run it via `task: spawn`:

```json
[
  { "label": "maven: dependency tree", "command": "mvn dependency:tree -B", "reveal": "always" }
]
```

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

## License

MIT
