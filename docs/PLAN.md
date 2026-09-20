# Zed 编辑器 Maven / Spring Boot 支持插件 —— 分析与实施计划

> **Scope update (2026-09-20):** the shipped extension now focuses on Maven
> only (`maven` / zed-maven). The Spring Boot phases (M3+) below are kept as
> research notes for a possible separate extension.

> 目标:在 Zed 中获得接近 IDEA 的 pom.xml 编辑体验与 Spring Boot 开发支持。
> 调研环境:Zed 1.20.2 / macOS / JDK 21(jenv)/ Node 24
> 调研日期:2026-09

---

## 1. 现状调研结论

### 1.1 Zed 官方扩展生态的空白

官方扩展库(zed-industries/extensions)目前:

| 能力 | 现状 |
|---|---|
| XML 语言 | ❌ 无任何扩展(无语法、无 LSP) |
| Maven / pom.xml | ❌ 无 |
| Spring Boot | ❌ 无 |
| Java | ✅ `java`(官方)与 `java-eclipse-jdtls`(社区,jdt.ls,自带 Properties 语法) |

**结论:这是一个完全空白的领域,做出的东西有直接发布价值。**

### 1.2 Zed 扩展体系能做什么(2026, schema_version 1)

| 能力 | 机制 | 对本项目的意义 |
|---|---|---|
| 语法高亮/大纲/折叠/缩进 | tree-sitter grammar + `.scm` 查询 | XML / pom.xml / properties |
| LSP 接入 | `extension.toml` 声明 + Rust 实现下载/启动命令 | LemMinX、Spring Boot LS |
| 一个语言挂多个 LSP | settings `"language_servers": [...]` | jdt.ls + Spring LS 共存 |
| Runnable 检测 | `runnables.scm`(`@run` capture + `ZED_CUSTOM_*` 环境变量) | 主类/插件旁"运行"按钮 |
| Snippets | `snippets/*.json` | pom 骨架、常用配置 |
| Slash Command / MCP Context Server | 扩展 API | `mvn` 目标执行、Spring Initializr 生成 |
| ❌ 无 UI 面板 API | — | 无法做 IDEA 的 Beans 视图、依赖树图、Effective POM 对话框 |

### 1.3 关键技术选型验证结果

**XML 语法**:`tree-sitter-grammars/tree-sitter-xml`(活跃维护,2026-09 仍有提交,52 stars)✅

**XML/Maven LSP —— Eclipse LemMinX + lemminx-maven**:
- LemMinX(eclipse-lemminx/lemminx):成熟 XML LSP——schema 校验、标签/属性补全、格式化、hover、折叠;内置常见文件的 schema 关联(**pom.xml → Maven 官方 XSD**,即使不装 maven 插件也有补全)
- lemminx-maven(eclipse-lemminx/lemminx-maven,Eclipse 官方,活跃):
  - `groupId:artifactId:version` 依赖补全(本地 `~/.m2` 仓库 + Maven Central 在线搜索)
  - 版本补全、依赖 hover(显示描述/最新版)
  - 通过 classpath 或扩展 jar 机制注入 LemMinX
- ⚠️ **分发渠道约束(已实测)**:两者 GitHub Releases 均无二进制资产;`org.eclipse.lemminx` 也不在 Maven Central。→ **需自建 CI 产出 uber jar,发布到我们扩展仓库的 GitHub Releases,扩展运行时下载**

**Spring Boot LSP —— spring-boot-language-server(spring-projects/sts4)**:
- VS Code "Spring Boot Tools" 背后的同一引擎,纯 LSP,编辑器无关
- 核心能力:
  - `application.properties` / `application.yml`:基于 classpath 中 `spring-configuration-metadata.json` 的**属性名补全、校验、hover 文档**(这是脱离 VS Code 也 100% 可用的强项)
  - Java 侧:Spring 注解补全、bean/endpoint 导航(部分能力在 VS Code 中依赖 jdt.ls 协同,standalone 下是子集)
  - 官方提供 **`spring-boot-language-server-standalone`** 模块(mainClass `StandaloneBootApp`),专为非 VS Code 客户端设计(Neovim 社区即用此方案)
- ⚠️ 同样无官方预编译 jar(VS Code 市场只打包进 vsix)→ 同样走自建 CI + 自有 Releases

**运行前提**:所有 Java LS 需要 JVM。用户有 JDK 21,但注意 **GUI 启动的 Zed 不继承 jenv/shell PATH** → 扩展需实现 java 探测链:`worktree.which("java")` → `JAVA_HOME` → `/usr/libexec/java_home`(macOS)→ 常见 Homebrew 路径。

### 1.4 与 IDEA 的能力对标(可行性判定)

| IDEA 功能 | Zed 可行性 | 方案 |
|---|---|---|
| pom.xml 语法高亮/折叠/大纲 | ✅ | tree-sitter-xml + queries |
| pom.xml 标签/属性 schema 补全、校验 | ✅ | LemMinX 内置 Maven XSD |
| 依赖坐标补全(本地仓库 + Central) | ✅ | lemminx-maven |
| 依赖版本补全 / hover 详情 | ✅ | lemminx-maven |
| 跳到依赖源码/javadoc | 🟡 | jdt.ls 负责纯 Java 侧;pom 内跳 jar 源码暂缺 |
| application.properties/yml 补全校验 | ✅ | spring-boot-language-server |
| Spring bean 导航/注入点提示 | 🟡 | Spring LS standalone 子集 + jdt.ls |
| 编辑器内运行 Boot 应用 / Maven goal | 🟡 | runnables(`@SpringBootApplication` 主类)+ task 模板(`mvn spring-boot:run`) |
| 依赖树图 / Effective POM 视图 / Beans 面板 | ❌(无 UI API) | 变通:`mvn dependency:tree` slash command 或 task |
| Spring Initializr 新建项目 | 🟡 | slash command `/spring init`(start.spring.io API) |
| Actuator live 信息 | ❌ | — |

---

## 2. 架构设计

### 2.1 仓库拆分(官方扩展库要求 1 repo = 1 扩展)

```
┌─────────────────────────────────────────────────────────────┐
│ zed-xml          XML 语言(通用,非 Java 用户也受益)          │
│   ├─ tree-sitter-xml 语法 + 全套 queries                     │
│   └─ LSP: LemMinX(schema 校验/补全/格式化)                  │
├─────────────────────────────────────────────────────────────┤
│ zed-maven        Maven 深度支持(pom.xml)                    │
│   └─ LSP: LemMinX + lemminx-maven(classpath 注入)           │
│      → 也挂在 "XML" 语言上,与 zed-xml 的 LemMinX 二选一启用  │
├─────────────────────────────────────────────────────────────┤
│ zed-spring-boot  Spring Boot 支持                            │
│   ├─ LSP: spring-boot-language-server (standalone)           │
│   │    → 挂 "Java"(与 jdt.ls 共存)+"Properties"+"YAML"     │
│   ├─ snippets: pom 骨架/常用依赖/常用 properties              │
│   ├─ runnables: Boot 主类运行按钮                            │
│   └─ (可选) slash command: /spring init、/mvn <goal>         │
└─────────────────────────────────────────────────────────────┘

共享构建仓库:ls-jars(GitHub Actions 构建 LemMinX uber jar、
  LemMinX+lemminx-maven 组合 jar、spring-boot-LS standalone jar,
  发布到各自扩展仓库的 Releases)
```

**起步策略**:先做单仓库 all-in-one(本地 `zed --install-extension` 开发验证),技术走通后再按上图拆分发布。避免一开始维护 4 个 repo。

### 2.2 单扩展内部结构(以 zed-spring-boot 为例)

```
zed-spring-boot/
  extension.toml        # 声明语言服务器;languages = ["Java","Properties","YAML"]
  Cargo.toml / src/lib.rs   # java 探测、jar 下载(Releases)、启动命令拼装
  languages/
    properties/         # 若 jdtls 扩展未装则由本扩展提供(Properties 语法树)
      config.toml
  snippets/spring-boot.json
  runnables: languages/java/runnables.scm(@SpringBootApplication 主类)
  README.md             # 安装说明 + tasks.json 模板(mvn test/package/bootRun)
```

LSP 启动命令(核心逻辑):

```
java -Xmx512m \
     -cp lemminx-uber.jar:lemminx-maven.jar org.eclipse.lemminx.XMLServerLauncher   # XML
java -Xmx1g -jar spring-boot-language-server-standalone.jar                          # Spring
```

> LemMinX 与 lemminx-maven 合并进同一 JVM;Spring LS 独立 JVM;jdt.ls(若装)第三个 JVM —— 通过 Zed 按需启动机制,只有打开对应文件才会拉起对应进程。

### 2.3 需要自建的 CI(ls-jars)

```
GitHub Actions:
  1. lemminx:     git clone → mvn -P uber package → org.eclipse.lemminx-uber.jar
  2. lemminx-maven: clone → mvn package(依赖 1 的产物)→ 合并 classpath 清单
  3. spring-boot-ls: clone sts4 → headless-services 构建链(官方 build.sh,
     需 JDK17+、可能需 xvfb)→ spring-boot-language-server-standalone jar
  → 各自触发 release,产物附 license/notice(EPL-2.0 合规)
```

---

## 3. 分阶段里程碑

### M0 —— XML 语言基础(0.5~1 天)
- [ ] 扩展骨架(`zed extension new` 模式)、tree-sitter-xml 接入
- [ ] highlights / outline / fold / brackets / indents queries
- [ ] 本地 dev 安装,用真实 pom.xml / settings.xml 验收
- **交付物:pom.xml 在 Zed 里"长得像样"**

### M1 —— LemMinX 接入(1~2 天)
- [ ] ls-jars CI 产出 lemminx uber jar → 发布 Release
- [ ] Rust 端:下载缓存、java 探测链、`language_server_command`
- [ ] 验证:pom.xml XSD 校验(错误波浪线)、标签/属性补全、格式化
- **交付物:pom.xml 基本编辑体验 ≈ IDEA(编辑辅助层面)**

### M2 —— Maven 深度(1~2 天)
- [ ] lemminx-maven 打包注入;Zed `LspSettings.initialization_options` 透传
- [ ] 验证:依赖 GAV 补全(本地 + Central 在线)、版本补全、hover
- [ ] 兜底:Central 搜索 API 不稳时的本地仓库补全(已实测 search.maven.org 有限流)
- **交付物:加依赖不用切浏览器查 Maven Central**

### M3 —— Spring Boot LS(2~4 天,风险最高)
- [ ] CI 构建 standalone jar(sts4 构建链较重,先本地跑通再上 CI)
- [ ] 扩展声明多语言挂载;Java 双 LS(jdt.ls + Spring LS)共存验证
- [ ] 验证:application.properties/yml 补全、校验、hover(用最小 Boot 项目 testdata)
- [ ] Properties 语言冲突处理:检测 jdtls 扩展已提供则复用;否则自带 tree-sitter-properties
- **交付物:配置文件编辑体验 ≈ IDEA**

### M4 —— 编辑器体验层(1~2 天)
- [ ] snippets:pom 骨架、常用 dependency、常用 properties 项
- [ ] runnables:`@SpringBootApplication` 主类 → 运行按钮(`ZED_CUSTOM_*` + task)
- [ ] README 附 tasks 模板:test / package / spring-boot:run / devtools
- **交付物:常用操作不离开 Zed**

### M5 —— 增值与发布(可选,各 1~2 天)
- [ ] `/spring init` slash command(start.spring.io API 生成项目)
- [ ] `/mvn` slash command 或 MCP context server(执行 goal、dependency:tree,参考你已有 zed-mcp-server-redmine 经验)
- [ ] 拆分 3 repo → PR 至 zed-industries/extensions(审核发布)
- [ ] (远期)Gradle 支持、依赖源码跳转

---

## 4. 风险与对策

| 风险 | 影响 | 对策 |
|---|---|---|
| GUI 启动的 Zed 找不到 java(jenv PATH 不继承) | LS 无法启动 | java 探测链 + README 指导 `launchctl setenv JAVA_HOME` |
| 三个 JVM 内存(lemminx ~200M、Spring LS ~500M+、jdt.ls ~1-2G) | 内存压力 | Zed 按需启动;`-Xmx` 限制;文档说明按需禁用某 LS |
| sts4 standalone 构建复杂(xvfb、多模块) | M3 延期 | 先本地构建走通;备选:从 VS Code vsix 提取 jar(非官方渠道,仅兜底) |
| Spring LS Java 侧功能在无 jdt.ls 协同时是子集 | 期望管理 | README 明示:properties/yml 是完全体,Java 导航部分依赖 jdt.ls |
| LemMinX/spring LS 私有 LSP 扩展 Zed 不支持 | 功能打折 | 无害降级;properties/completion 等标准能力不受影响 |
| EPL-2.0 再分发合规 | 法律 | Release 产物附 LICENSE/NOTICE;版本升级脚本化 |
| 扩展 API 演进 | 维护成本 | 锁定 zed_extension_api 版本;跟随 schema_version 更新 |
| search.maven.org 限流(已实测) | 在线补全质量 | 本地 `~/.m2` 补全为主,在线为辅 |

---

## 5. 快速验证清单(动手前 30 分钟可完成的可行性 spike)

1. `git clone tree-sitter-grammars/tree-sitter-xml` → 最小 extension.toml → dev 安装 → 看 pom.xml 高亮 ✅(语法)
2. 本地 `mvn package` 构建 lemminx + lemminx-maven → 手工 `java -cp ... XMLServerLauncher` 以 stdio 模式起服务 → 确认启动参数与 initializationOptions ✅(LSP)
3. 本地构建 sts4 standalone jar → stdio 起 → 用 `spring-boot-properties` 测试工程验证 properties 补全 ✅(Spring LS)
4. 三者分别用 Zed 本地扩展方式接入验证

任何一步失败都有备选(如 Spring LS 可降级为仅 properties 支持的自建轻量 LS),不会推翻整体架构。
