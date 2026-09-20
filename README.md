# zed-maven

[English](#english) | [中文](#中文)

---

<a name="english"></a>

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

---

<a name="中文"></a>

# zed-maven(中文)

为 [Zed 编辑器](https://zed.dev)提供 [Maven](https://maven.apache.org)
`pom.xml` 智能支持。

## 功能矩阵

在 `pom.xml` 上运行 [LemMinX](https://github.com/eclipse-lemminx/lemminx)
(含 [lemminx-maven](https://github.com/eclipse-lemminx/lemminx-maven) 扩展):

| 功能 | 状态 | 说明 |
|---|---|---|
| Schema 校验(Maven XSD) | ✅ | 对非法标签与错误嵌套即时报告诊断 |
| 标签与属性补全 | ✅ | 基于 schema |
| 依赖 `<groupId>` 补全 | ✅ | 来源为本地 `~/.m2` 仓库 |
| 依赖 `<artifactId>` 补全 | ✅ | 带完整坐标;选中后自动填充同级标签 |
| 依赖 `<version>` 补全 | ✅ | 列出 artifact 可用版本 |
| `<scope>` / 执行 `<phase>` 补全 | ✅ | 枚举值 |
| Maven 属性 `${...}` 补全 | ✅ | 基于 effective model 中的属性 |
| 悬停详情(hover) | ✅ | 标签文档(XSD)+ artifact 信息 |
| 代码操作(Code Actions) | ✅ | 提取/内联 `${property}`、移除受管版本、移除坐标片段 |
| Maven 属性重命名 | ✅ | |
| 属性跳转定义 | ✅ | |
| XML 格式化 | ✅ | LemMinX 内置 |
| Maven Central 远程搜索 | ⚠️ | 可选,依赖网络 |
| 依赖树 | ✅ | 通过 Zed 任务,见下文 |

语法高亮由官方 `xml` 扩展提供;请一并安装。

### 依赖树(任务)

添加到项目 `.zed/tasks.json`,通过 `task: spawn` 运行:

```json
[
  { "label": "maven: 依赖树", "command": "mvn dependency:tree -B", "reveal": "always" }
]
```

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

## 许可证

MIT
