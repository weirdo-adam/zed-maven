# zed-spring-suite

Maven & Spring Boot support for the [Zed editor](https://zed.dev): one extension
covering XML syntax, Maven `pom.xml` intelligence, and (planned) Spring Boot
language-server integration.

> Roadmap and research notes: see [PLAN.md](./PLAN.md).

## Status

- [x] **M0 — XML language**: syntax highlighting, outline (structural elements
      only), bracket matching & rainbow control, auto-indent, tag auto-close,
      comment toggling (`<!-- -->`) for `.xml`, `.xsd`, `.xsl`, `.xslt`,
      `.xhtml`, `.wsdl`, `.svg` — including `pom.xml`, `settings.xml`, etc.
- [ ] **M1 — LemMinX language server**: schema-aware completion & validation
      for `pom.xml` (Maven XSD).
- [ ] **M2 — lemminx-maven**: dependency `groupId/artifactId/version`
      completion (local `~/.m2` + Maven Central), hover info.
- [ ] **M3 — spring-boot-language-server**: `application.properties`/`.yml`
      completion & validation, Spring Java support.
- [ ] **M4 — Snippets & runnables**: pom skeletons, `@SpringBootApplication`
      run button, task templates.

## Install (development)

1. Open Zed → command palette → `zed: install dev extension`
   (Extensions page → *Install Dev Extension*)
2. Select this repository's root directory.
3. Open [`testdata/pom.xml`](./testdata/pom.xml) to verify highlighting,
   outline (`cmd-shift-o` / outline panel), folding and auto-indent.

## License

MIT
