# zed-maven

[English](#english) | [中文](#中文)

---

<a name="english"></a>

[Maven](https://maven.apache.org) `pom.xml` intelligence for the
[Zed editor](https://zed.dev).

## Features

Runs [LemMinX](https://github.com/eclipse-lemminx/lemminx) with
[lemminx-maven](https://github.com/eclipse-lemminx/lemminx-maven) on `pom.xml`:

- Schema-aware validation & completion (Maven XSD)
- Dependency completion: `<groupId>` / `<artifactId>` (with
  `group:artifact:version` labels, auto-fills sibling tags) / `<version>`
- Hover details on dependencies

Syntax highlighting is provided by the official `xml` extension; install it alongside.

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
  `null`; Zed cannot deserialize it, and the fix also enables
  `additionalTextEdits` for dependency completion).
- `.github/workflows/release-servers.yml` — packages and publishes the jars.
- `scripts/smoke.py` — LSP smoke test over the packaged jars:

  ```sh
  python3 scripts/smoke.py --java <java> \
    --lemminx-jar <lemminx-uber.jar> --ext-zip <lemminx-maven-deps.zip>
  ```

## License

MIT

---

<a name="中文"></a>

# zed-maven(中文)

为 [Zed 编辑器](https://zed.dev)提供 [Maven](https://maven.apache.org)
`pom.xml` 智能支持。

## 功能

在 `pom.xml` 上运行 [LemMinX](https://github.com/eclipse-lemminx/lemminx)
(含 [lemminx-maven](https://github.com/eclipse-lemminx/lemminx-maven) 扩展):

- 基于 Maven XSD 的校验与标签补全(对非法标签与错误嵌合即时报告诊断)
- 依赖补全:`<groupId>` / `<artifactId>`(带
  `group:artifact:version` 完整坐标,选中后自动填充同级标签)/ `<version>`
- 依赖项悬停详情(hover)

语法高亮由官方 `xml` 扩展提供——请一并安装。

## 安装(开发模式)

1. 在 Zed 扩展页安装官方 `xml` 扩展。
2. 命令面板 → `zed: install dev extension` → 选择本仓库目录。
3. 打开 Maven 项目的 `pom.xml`。首次启动会自动从
   [releases](https://github.com/weirdo-adam/zed-maven/releases)
   下载服务器 jar(约 19 MB)。

依赖:JDK 17+。`java` 查找顺序:工作区 PATH → `ZED_JAVA_HOME` →
`JAVA_HOME` → 常见 Homebrew/JVM 安装路径。也可设置 `ZED_LEMMINX_HOME`
指向手工准备的目录(`org.eclipse.lemminx-uber.jar` + `maven-ext/`)跳过
Release 下载。

## 开发

- `patches/` — LemMinX 基于 0.31.2 tag 应用
  `lemminx-resolve-null.patch` 构建(`completionItem/resolve` 不能返回
  `null`,Zed 无法反序列化;修复同时恢复了依赖补全的
  `additionalTextEdits` 能力)。
- `.github/workflows/release-servers.yml` — 打包并发布服务器 jar。
- `scripts/smoke.py` — 对发布产物做 LSP 冒烟测试:

  ```sh
  python3 scripts/smoke.py --java <java路径> \
    --lemminx-jar <lemminx-uber.jar> --ext-zip <lemminx-maven-deps.zip>
  ```

## 许可证

MIT
