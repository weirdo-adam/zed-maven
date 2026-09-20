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
| Schema-aware completion & validation for `pom.xml` (LemMinX) | **spring-boot** | ✅ M1 (dev) |
| Dependency `groupId/artifactId/version` completion (lemminx-maven) | **spring-boot** | ✅ M1 (dev) |
| `application.properties`/`.yml` intelligence (spring-boot-language-server) | **spring-boot** | 🚧 M3 |
| Snippets & `@SpringBootApplication` runnables | **spring-boot** | 🚧 M4 |

`contrib/upstream-zed-xml/` holds extra/validated queries (e.g.
`brackets.scm`) intended to be contributed upstream.

## Install (development)

1. Install the `xml` extension from Zed's extension page.
2. Command palette → `zed: install dev extension` → select this directory.
3. Open [`testdata/pom.xml`](./testdata/pom.xml).

> Note: the dev-extension install registers the directory path, so after the
> earlier rename you'll need to re-install the dev extension from
> `~/Developer/github/zed-spring-boot`.

### Running the language server locally (M1)

The extension runs the servers as `java -cp "lemminx-uber.jar:maven-ext/*"
org.eclipse.lemminx.XMLServerLauncher`. Until CI publishes jars to GitHub
releases, point `ZED_LEMMINX_HOME` at a prepared directory:

```
server/
├── org.eclipse.lemminx-uber.jar   # LemMinX (Eclipse releases or `mvn package`)
└── maven-ext/                     # lemminx-maven + its 50 dependency jars
```

Build the jars locally (requires JDK 17+ and Maven):

```sh
# LemMinX uber jar
git clone --depth 1 https://github.com/eclipse-lemminx/lemminx
(cd lemminx && mvn -DskipTests -Dcbi.jarsigner.skip=true package)
cp lemminx/org.eclipse.lemminx/target/org.eclipse.lemminx-uber.jar server/

# lemminx-maven with dependencies
git clone --depth 1 https://github.com/eclipse-lemminx/lemminx-maven
(cd lemminx-maven && mvn -DskipTests package)
unzip lemminx-maven/lemminx-maven/target/*-zip-with-dependencies.zip -d server/maven-ext
cp lemminx-maven/lemminx-maven/target/lemminx-maven-*.jar server/maven-ext/
```

Then launch Zed with the variable set (GUI launches don't inherit your shell
environment — either start Zed from a terminal or use `launchctl setenv`):

```sh
ZED_LEMMINX_HOME=$PWD/server zed   # from this repo root
# or, for GUI launches:
launchctl setenv ZED_LEMMINX_HOME ~/Developer/github/zed-spring-boot/server
```

Requirements: JDK 17+ (`brew install openjdk@21`); `java` is resolved via
worktree PATH → `ZED_JAVA_HOME` → `JAVA_HOME` → common Homebrew/JVM locations.

Verified capabilities (smoke-tested over LSP stdio): schema validation of
`pom.xml`, `<groupId>` completion (649 items from local `~/.m2`),
`<artifactId>` completion with full `group:artifact:version` labels (2600+
items), hover, document symbols.

## License

MIT
