# 基礎編 part2(随時更新予定)

前回 sdkman が使いやすいというお話を致しました。

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45058742" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45058742">オデッセイ / 星界</a></iframe>

PV 可愛すぎる  
星界ちゃんとお出かけしたいよね。

# gradle で環境構築をしてみる

`gradle init`を実行すると、最初に

- Application(実行ファイル)
- Library(ライブラリ)

を聞かれる。(あと Basic と Gradle plugin があるがここでは省略)
実際に生成してみて違いを確かめてみた。

<details><summary>アプリケーションの場合</summary>

```
>gradle init

Select type of build to generate:
1: Application
2: Library
3: Gradle plugin
4: Basic (build structure only)
Enter selection (default: Application) [1..4] 1

Select implementation language:
1: Java
2: Kotlin
3: Groovy
4: Scala
5: C++
6: Swift
Enter selection (default: Java) [1..6] 2

Enter target Java version (min: 7, default: 21): 21

Project name (default: tmp.q5kXPDaYCv):

Select application structure:
1: Single application project
2: Application and library project
Enter selection (default: Single application project) [1..2] 1

Select build script DSL:
1: Kotlin
2: Groovy
Enter selection (default: Kotlin) [1..2] 1

Select test framework:
1: kotlin.test
2: JUnit Jupiter
Enter selection (default: kotlin.test) [1..2] 1

Generate build using new APIs and behavior (some features may change in the next minor release)? (default: no) [yes, no]

> Task :init
> Learn more about Gradle by exploring our Samples at https://docs.gradle.org/8.14.2/samples/sample_building_kotlin_applications.html

BUILD SUCCESSFUL in 13s
1 actionable task: 1 executed

```

```
>tree
.
├── app
│   ├── build.gradle.kts
│   └── src
│       ├── main
│       │   ├── kotlin
│       │   │   └── org
│       │   │       └── example
│       │   │           └── App.kt
│       │   └── resources
│       └── test
│           ├── kotlin
│           │   └── org
│           │       └── example
│           │           └── AppTest.kt
│           └── resources
├── build
│   └── reports
│       └── configuration-cache
│           ├── 4oniusdp5r7fugbijssf0wr1j
│           │   └── 5q136g6kyf5rmfirte8rxuy4s
│           │       └── configuration-cache-report.html
│           └── ey2md2b405fd5q446rzq48txv
│               └── 74ftzik9peji2a08l95gz9c0j
│                   └── configuration-cache-report.html
├── gradle
│   ├── libs.versions.toml
│   └── wrapper
│       ├── gradle-wrapper.jar
│       └── gradle-wrapper.properties
├── gradle.properties
├── gradlew
├── gradlew.bat
└── settings.gradle.kts

22 directories, 12 files
```

</details>

<details><summary>ライブラリプロジェクトの場合</summary>

```

> gradle init

Select type of build to generate:
1: Application
2: Library
3: Gradle plugin
4: Basic (build structure only)
Enter selection (default: Application) [1..4] 2

Select implementation language:
1: Java
2: Kotlin
3: Groovy
4: Scala
5: C++
6: Swift
Enter selection (default: Java) [1..6] 2

Enter target Java version (min: 7, default: 21): 21

Project name (default: tmp.vVJk4Hd6GU):

Select build script DSL:
1: Kotlin
2: Groovy
Enter selection (default: Kotlin) [1..2] 1

Select test framework:
1: kotlin.test
2: JUnit Jupiter
Enter selection (default: kotlin.test) [1..2] 1

Generate build using new APIs and behavior (some features may change in the next minor release)? (default: no) [yes, no]

> Task :init
> Learn more about Gradle by exploring our Samples at https://docs.gradle.org/8.14.2/samples/sample_building_kotlin_libraries.html

BUILD SUCCESSFUL in 9s
1 actionable task: 1 executed

```

```
.
├── gradle
│   ├── libs.versions.toml
│   └── wrapper
│       ├── gradle-wrapper.jar
│       └── gradle-wrapper.properties
├── gradle.properties
├── gradlew
├── gradlew.bat
├── lib
│   ├── build.gradle.kts
│   └── src
│       ├── main
│       │   ├── kotlin
│       │   │   └── org
│       │   │       └── example
│       │   │           └── Library.kt
│       │   └── resources
│       └── test
│           ├── kotlin
│           │   └── org
│           │       └── example
│           │           └── LibraryTest.kt
│           └── resources
└── settings.gradle.kts

15 directories, 10 files

```

</details>

## 生成されたソースファイル

`cat app/src/main/kotlin/org/example/App.kt`

```kotlin
/*
 * This source file was generated by the Gradle 'init' task
 */
package org.example

class App {
    val greeting: String
        get() {
            return "Hello World!"
        }
}

fun main() {
    println(App().greeting)
}

```

`cat lib/src/main/kotlin/org/example/Library.kt`

```kotlin
/*
 * This source file was generated by the Gradle 'init' task
 */
package org.example

class Library {
    fun someLibraryMethod(): Boolean {
        return true
    }
}

```

rust 同様、エントリーポイントになる main 関数が存在しない場合は Library となりそうだ。

## kotlin.test もちょっと気になる

```kotlin
/*
 * This source file was generated by the Gradle 'init' task
 */
package org.example

import kotlin.test.Test
import kotlin.test.assertNotNull

class AppTest {
    @Test fun appHasAGreeting() {
        val classUnderTest = App()
        assertNotNull(classUnderTest.greeting, "app should have a greeting")
    }
}
```

kotlin-test は kotest と改名された？らしい[^参考2] [^参考3]

と思ったけど build.gradle 見てみたら

```
    // Use the Kotlin JUnit 5 integration.
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")

    // Use the JUnit 5 integration.
    testImplementation(libs.junit.jupiter.engine)

```

JUnit やったし。  
JUnit の特徴として、アノテーションが挙げられるらしい？ kotest は[BDD 形式](https://zenn.dev/amanotakeshi/articles/03184f9316b416)というのが用いられてるとのこと。[^参考1]

### そもそも unit test ってなんだ。

**単体テスト**。

> [!NOTE]
> unit は単体、一人、一個、単位などを意味する名刺[^参考4]

unit ってまとまったみたいな意味を持ってるのかと思ってたわ。  
統合テスト、これは英語だと integration test になるらしい。 integration は統合って意味を持つのか。  
漢字読み間違えてた。　結合じゃなくて統合か。

rust は統合テストを`tests`ディレクトリに、単体テストは大抵同じソースコードに書くけど、  
kotlin(junit)は統合テストも単体テストもディレクトリが別れてるんだね。  
あ、こら、rust が特殊とか言わない!

> [!TIP]
> この記事は決して日記さんがあふぉなことを証明するものではありません! ...多分

# できることを知っておく

```

> gradle task
> Calculating task graph as no cached configuration is available for tasks: task

> Task :tasks

---

## Tasks runnable from root project 'tmp.q5kXPDaYCv'

## Application tasks

run - Runs this project as a JVM application

## Build tasks

assemble - Assembles the outputs of this project.
build - Assembles and tests this project.
buildDependents - Assembles and tests this project and all projects that depend on it.
buildKotlinToolingMetadata - Build metadata json file containing information about the used Kotlin tooling
buildNeeded - Assembles and tests this project and all projects it depends on.
classes - Assembles main classes.
clean - Deletes the build directory.
jar - Assembles a jar archive containing the classes of the 'main' feature.
kotlinSourcesJar - Assembles a jar archive containing the sources of target 'kotlin'.
testClasses - Assembles test classes.

## Build Setup tasks

init - Initializes a new Gradle build.
updateDaemonJvm - Generates or updates the Gradle Daemon JVM criteria.
wrapper - Generates Gradle wrapper files.

## Distribution tasks

assembleDist - Assembles the main distributions
distTar - Bundles the project as a distribution.
distZip - Bundles the project as a distribution.
installDist - Installs the project as a distribution as-is.

## Documentation tasks

javadoc - Generates Javadoc API documentation for the 'main' feature.

## Help tasks

artifactTransforms - Displays the Artifact Transforms that can be executed in root project 'tmp.q5kXPDaYCv'.
buildEnvironment - Displays all buildscript dependencies declared in root project 'tmp.q5kXPDaYCv'.
dependencies - Displays all dependencies declared in root project 'tmp.q5kXPDaYCv'.
dependencyInsight - Displays the insight into a specific dependency in root project 'tmp.q5kXPDaYCv'.
help - Displays a help message.
javaToolchains - Displays the detected java toolchains.
kotlinDslAccessorsReport - Prints the Kotlin code for accessing the currently available project extensions and conventions.
outgoingVariants - Displays the outgoing variants of root project 'tmp.q5kXPDaYCv'.
projects - Displays the sub-projects of root project 'tmp.q5kXPDaYCv'.
properties - Displays the properties of root project 'tmp.q5kXPDaYCv'.
resolvableConfigurations - Displays the configurations that can be resolved in root project 'tmp.q5kXPDaYCv'.
tasks - Displays the tasks runnable from root project 'tmp.q5kXPDaYCv' (some of the displayed tasks may belong to subprojects).

## Verification tasks

check - Runs all checks.
checkKotlinGradlePluginConfigurationErrors - Checks that Kotlin Gradle Plugin hasn't reported project configuration errors, failing otherwise. This task always runs before compileKotlin\* or similar tasks.
test - Runs the test suite.

To see all tasks and more detail, run gradle tasks --all

To see more detail about a task, run gradle help --task <task>

BUILD SUCCESSFUL in 6s
1 actionable task: 1 executed
Configuration cache entry stored.

```

`gradle tasks` はタスクというのを表示する。 tasks って自分で追加することもできるらしい。

## 気になった task 探索 1: javaToolchains

```
>gradle javaToolchains
Calculating task graph as no cached configuration is available for tasks: javaToolchains

> Task :javaToolchains

 + Options
     | Auto-detection:     Enabled
     | Auto-download:      Enabled

 + Azul Zulu JDK 1.8.0_432-b06
     | Location:           /Library/Java/JavaVirtualMachines/zulu-8.jdk/Contents/Home
     | Language Version:   8
     | Vendor:             Azul Zulu
     | Architecture:       aarch64
     | Is JDK:             true
     | Detected by:        MacOS java_home

 + Azul Zulu JDK 17.0.13+11-LTS
     | Location:           /Library/Java/JavaVirtualMachines/zulu-17.jdk/Contents/Home
     | Language Version:   17
     | Vendor:             Azul Zulu
     | Architecture:       aarch64
     | Is JDK:             true
     | Detected by:        Current JVM

 + Azul Zulu JDK 21.0.5+11-LTS
     | Location:           /Library/Java/JavaVirtualMachines/zulu-21.jdk/Contents/Home
     | Language Version:   21
     | Vendor:             Azul Zulu
     | Architecture:       aarch64
     | Is JDK:             true
     | Detected by:        MacOS java_home

 + Azul Zulu JDK 23.0.1+11
     | Location:           /Library/Java/JavaVirtualMachines/zulu-23.jdk/Contents/Home
     | Language Version:   23
     | Vendor:             Azul Zulu
     | Architecture:       aarch64
     | Is JDK:             true
     | Detected by:        MacOS java_home

 + Microsoft JDK 21.0.7+6-LTS
     | Location:           /Library/Java/JavaVirtualMachines/microsoft-21.jdk/Contents/Home
     | Language Version:   21
     | Vendor:             Microsoft
     | Architecture:       aarch64
     | Is JDK:             true
     | Detected by:        MacOS java_home


BUILD SUCCESSFUL in 814ms
1 actionable task: 1 executed
Configuration cache entry stored.
```

ほへぇシステムにある Java 自動で検知してきてくれるんだ。 java_home で探したとも書かれてるね。

## 気になった task 探索 2: projects

```
>gradle projects
<Calculating task graph as no cached configuration is available for tasks: projects

> Task :projects

Projects:

------------------------------------------------------------
Root project 'tmp.vVJk4Hd6GU'
------------------------------------------------------------

Root project 'tmp.vVJk4Hd6GU'
\--- Project ':lib'

To see a list of the tasks of a project, run gradle <project-path>:tasks
For example, try running gradle :lib:tasks

BUILD SUCCESSFUL in 374ms
1 actionable task: 1 executed
Configuration cache entry stored.
```

プロジェクトが設定できるらしい。
試しに lib を lib2 として複製してみる。 これだけだと認識しない。

```diff
>cat settings.gradle.kts
/*
 * This file was generated by the Gradle 'init' task.
 *
 * The settings file is used to specify which projects to include in your build.
 * For more detailed information on multi-project builds, please refer to https://docs.gradle.org/8.14.2/userguide/multi_project_builds.html in the Gradle documentation.
 */

plugins {
    // Apply the foojay-resolver plugin to allow automatic download of JDKs
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.10.0"
}

rootProject.name = "tmp.vVJk4Hd6GU"
include("lib")
+ include("lib2")
```

`settings.gradle.kts`にそれっぽいこと(`include`)が書かれてるので追記。  
結果、認識した

```
>gradle projects
Calculating task graph as configuration cache cannot be reused because file 'settings.gradle.kts' has changed.

> Task :projects

Projects:

------------------------------------------------------------
Root project 'tmp.vVJk4Hd6GU'
------------------------------------------------------------

Root project 'tmp.vVJk4Hd6GU'
+--- Project ':lib'
\--- Project ':lib2'

To see a list of the tasks of a project, run gradle <project-path>:tasks
For example, try running gradle :lib:tasks

BUILD SUCCESSFUL in 639ms
1 actionable task: 1 executed
Configuration cache entry stored.
```

なお、プロジェクトごとに task は異なる。  
ここに書いてあるように、`gradle <project-path>:tasks`で各プロジェクトの tasks 一覧を取得できる。  
`gradle tasks --all`を実行すると、すべての task を表示できる。

```diff
>gradle tasks --all
Calculating task graph as no cached configuration is available for tasks: tasks --all

> Task :tasks

------------------------------------------------------------
Tasks runnable from root project 'tmp.vVJk4Hd6GU'
------------------------------------------------------------

Build tasks
-----------
lib:assemble - Assembles the outputs of this project.
lib2:assemble - Assembles the outputs of this project.
lib:build - Assembles and tests this project.
lib2:build - Assembles and tests this project.
lib:buildDependents - Assembles and tests this project and all projects that depend on it.
lib2:buildDependents - Assembles and tests this project and all projects that depend on it.
lib:buildKotlinToolingMetadata - Build metadata json file containing information about the used Kotlin tooling
lib2:buildKotlinToolingMetadata - Build metadata json file containing information about the used Kotlin tooling
lib:buildNeeded - Assembles and tests this project and all projects it depends on.
lib2:buildNeeded - Assembles and tests this project and all projects it depends on.
lib:classes - Assembles main classes.
lib2:classes - Assembles main classes.
lib:clean - Deletes the build directory.
lib2:clean - Deletes the build directory.
lib:jar - Assembles a jar archive containing the classes of the 'main' feature.
lib2:jar - Assembles a jar archive containing the classes of the 'main' feature.
lib:kotlinSourcesJar - Assembles a jar archive containing the sources of target 'kotlin'.
lib2:kotlinSourcesJar - Assembles a jar archive containing the sources of target 'kotlin'.
lib:testClasses - Assembles test classes.
lib2:testClasses - Assembles test classes.

Build Setup tasks
-----------------
init - Initializes a new Gradle build.
updateDaemonJvm - Generates or updates the Gradle Daemon JVM criteria.
wrapper - Generates Gradle wrapper files.

Documentation tasks
-------------------
lib:javadoc - Generates Javadoc API documentation for the 'main' feature.
lib2:javadoc - Generates Javadoc API documentation for the 'main' feature.

Help tasks
----------
artifactTransforms - Displays the Artifact Transforms that can be executed in root project 'tmp.vVJk4Hd6GU'.
lib:artifactTransforms - Displays the Artifact Transforms that can be executed in project ':lib'.
lib2:artifactTransforms - Displays the Artifact Transforms that can be executed in project ':lib2'.
buildEnvironment - Displays all buildscript dependencies declared in root project 'tmp.vVJk4Hd6GU'.
lib:buildEnvironment - Displays all buildscript dependencies declared in project ':lib'.
lib2:buildEnvironment - Displays all buildscript dependencies declared in project ':lib2'.
dependencies - Displays all dependencies declared in root project 'tmp.vVJk4Hd6GU'.
lib:dependencies - Displays all dependencies declared in project ':lib'.
lib2:dependencies - Displays all dependencies declared in project ':lib2'.
dependencyInsight - Displays the insight into a specific dependency in root project 'tmp.vVJk4Hd6GU'.
lib:dependencyInsight - Displays the insight into a specific dependency in project ':lib'.
lib2:dependencyInsight - Displays the insight into a specific dependency in project ':lib2'.
help - Displays a help message.
lib:help - Displays a help message.
lib2:help - Displays a help message.
javaToolchains - Displays the detected java toolchains.
lib:javaToolchains - Displays the detected java toolchains.
lib2:javaToolchains - Displays the detected java toolchains.
lib:kotlinDslAccessorsReport - Prints the Kotlin code for accessing the currently available project extensions and conventions.
lib2:kotlinDslAccessorsReport - Prints the Kotlin code for accessing the currently available project extensions and conventions.
outgoingVariants - Displays the outgoing variants of root project 'tmp.vVJk4Hd6GU'.
lib:outgoingVariants - Displays the outgoing variants of project ':lib'.
lib2:outgoingVariants - Displays the outgoing variants of project ':lib2'.
projects - Displays the sub-projects of root project 'tmp.vVJk4Hd6GU'.
lib:projects - Displays the sub-projects of project ':lib'.
lib2:projects - Displays the sub-projects of project ':lib2'.
properties - Displays the properties of root project 'tmp.vVJk4Hd6GU'.
lib:properties - Displays the properties of project ':lib'.
lib2:properties - Displays the properties of project ':lib2'.
resolvableConfigurations - Displays the configurations that can be resolved in root project 'tmp.vVJk4Hd6GU'.
lib:resolvableConfigurations - Displays the configurations that can be resolved in project ':lib'.
lib2:resolvableConfigurations - Displays the configurations that can be resolved in project ':lib2'.
tasks - Displays the tasks runnable from root project 'tmp.vVJk4Hd6GU' (some of the displayed tasks may belong to subprojects).
lib:tasks - Displays the tasks runnable from project ':lib'.
lib2:tasks - Displays the tasks runnable from project ':lib2'.

Verification tasks
------------------
lib:check - Runs all checks.
lib2:check - Runs all checks.
lib:checkKotlinGradlePluginConfigurationErrors - Checks that Kotlin Gradle Plugin hasn't reported project configuration errors, failing otherwise. This task always runs before compileKotlin* or similar tasks.
lib2:checkKotlinGradlePluginConfigurationErrors - Checks that Kotlin Gradle Plugin hasn't reported project configuration errors, failing otherwise. This task always runs before compileKotlin* or similar tasks.
lib:test - Runs the test suite.
lib2:test - Runs the test suite.

Other tasks
-----------
lib:compileJava - Compiles main Java source.
lib2:compileJava - Compiles main Java source.
lib:compileKotlin - Compiles the compilation 'main' in target ''.
lib2:compileKotlin - Compiles the compilation 'main' in target ''.
lib:compileTestJava - Compiles test Java source.
lib2:compileTestJava - Compiles test Java source.
lib:compileTestKotlin - Compiles the compilation 'test' in target ''.
lib2:compileTestKotlin - Compiles the compilation 'test' in target ''.
components - Displays the components produced by root project 'tmp.vVJk4Hd6GU'. [deprecated]
lib:components - Displays the components produced by project ':lib'. [deprecated]
lib2:components - Displays the components produced by project ':lib2'. [deprecated]
dependentComponents - Displays the dependent components of components in root project 'tmp.vVJk4Hd6GU'. [deprecated]
lib:dependentComponents - Displays the dependent components of components in project ':lib'. [deprecated]
lib2:dependentComponents - Displays the dependent components of components in project ':lib2'. [deprecated]
+ lib2:hello
lib:mainClasses
lib2:mainClasses
model - Displays the configuration model of root project 'tmp.vVJk4Hd6GU'. [deprecated]
lib:model - Displays the configuration model of project ':lib'. [deprecated]
lib2:model - Displays the configuration model of project ':lib2'. [deprecated]
prepareKotlinBuildScriptModel
lib:processResources - Processes main resources.
lib2:processResources - Processes main resources.
lib:processTestResources - Processes test resources.
lib2:processTestResources - Processes test resources.

BUILD SUCCESSFUL in 467ms
1 actionable task: 1 executed
Configuration cache entry stored.
```

(せっかくなので lib2 に独自のタスクを設定してみた。)

## task を独自で定義する

```kotlin
tasks.register("hello") {
    doLast {
        println("Hello world!")
    }
}
```

[公式サイト](https://docs.gradle.org/current/userguide/tutorial_using_tasks.html) を元に定義、というかほぼコピー

register のコールバックとして、doLast を定義してると考えるのがいいのかね？

# マインクラフトと gradle

マインクラフトの mod 開発は、大抵 gralde となる。  
作るもの、使う前提 mod などによってセットアップが大きく異なる。

## paper の場合

gralde をライブラリで初期化し、次のページに書かれている内容を build.gradle.kts に追加する。
[Project setup | PaperMC Docs](https://docs.papermc.io/paper/dev/project-setup/)

ちなみに、kotlin で開発できる(と思う)  
kotlin にして問題があったら随時ページを更新します。

### 一例

```sh
>gradle init

Select type of build to generate:
  1: Application
  2: Library
  3: Gradle plugin
  4: Basic (build structure only)
Enter selection (default: Application) [1..4] 2

Select implementation language:
  1: Java
  2: Kotlin
  3: Groovy
  4: Scala
  5: C++
  6: Swift
Enter selection (default: Java) [1..6] 2

Enter target Java version (min: 7, default: 21): 21

Project name (default: tmp.1mPA8JuPth):

Select build script DSL:
  1: Kotlin
  2: Groovy
Enter selection (default: Kotlin) [1..2] 1

Select test framework:
  1: kotlin.test
  2: JUnit Jupiter
Enter selection (default: kotlin.test) [1..2] 1

Generate build using new APIs and behavior (some features may change in the next minor release)? (default: no) [yes, no]


> Task :init
Learn more about Gradle by exploring our Samples at https://docs.gradle.org/8.14.2/samples/sample_building_kotlin_libraries.html

BUILD SUCCESSFUL in 11s
1 actionable task: 1 executed
```

`cat build.gradle.kts`

```kotlin
/*
 * This file was generated by the Gradle 'init' task.
 *
 * This generated file contains a sample Kotlin library project to get you started.
 * For more details on building Java & JVM projects, please refer to https://docs.gradle.org/8.14.2/userguide/building_java_projects.html in the Gradle documentation.
 */

plugins {
    // Apply the org.jetbrains.kotlin.jvm Plugin to add support for Kotlin.
    alias(libs.plugins.kotlin.jvm)

    // Apply the java-library plugin for API and implementation separation.
    `java-library`
}

repositories {
    // Use Maven Central for resolving dependencies.
    mavenCentral()

    // Paper用に追加
    maven {
        name = "papermc"
        url = uri("https://repo.papermc.io/repository/maven-public/")
    }
}

dependencies {
    // Use the Kotlin JUnit 5 integration.
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")

    // Use the JUnit 5 integration.
    testImplementation(libs.junit.jupiter.engine)

    testRuntimeOnly("org.junit.platform:junit-platform-launcher")

    // This dependency is exported to consumers, that is to say found on their compile classpath.
    api(libs.commons.math3)

    // This dependency is used internally, and not exposed to consumers on their own compile classpath.
    implementation(libs.guava)

    // Paper用に追加
    compileOnly("io.papermc.paper:paper-api:1.21.8-R0.1-SNAPSHOT")
}

// Apply a specific Java toolchain to ease working on different environments.
java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

tasks.named<Test>("test") {
    // Use JUnit Platform for unit tests.
    useJUnitPlatform()
}


repositories {
}

dependencies {
  compileOnly("io.papermc.paper:paper-api:1.21.8-R0.1-SNAPSHOT")
}

java {
  toolchain.languageVersion.set(JavaLanguageVersion.of(21))
}
```

ここまでおわったら`gradle check`を実行しておく

### 必要そうな gradle プラグインを追加する

[shadow](https://gradleup.com/shadow/)というのが必要です。 fat-jar を生成します。  
依存関係を含め必要なファイルをすべて`jar`ファイルに保存します。[^参考7]

導入は簡単ですが、最新版を入れたら動作しなかったので、その辺は調整してください。

```kotlin
/*
 * This file was generated by the Gradle 'init' task.
 *
 * This generated file contains a sample Kotlin library project to get you started.
 * For more details on building Java & JVM projects, please refer to https://docs.gradle.org/8.14.2/userguide/building_java_projects.html in the Gradle documentation.
 */

plugins {
    // Apply the org.jetbrains.kotlin.jvm Plugin to add support for Kotlin.
    alias(libs.plugins.kotlin.jvm)

    // Apply the java-library plugin for API and implementation separation.
    `java-library`

    // shadow jarを導入
    id("com.gradleup.shadow") version "8.3.8"
}
```

<details><summary>最新版を入れた時に発生したエラー</summary>

9.0.0-rc2 で発生したエラー  
(検証日: 2025/07/28)

```
Caused by: java.lang.RuntimeException: Failed to remap plugin jar '/<個人情報を含むため省略>/tmp.1mPA8JuPth/lib/build/libs/lib-all.jar'
```

</details>

### 便利そうな gradle プラグインを追加する

[run-task](https://github.com/jpenilla/run-task)というのが便利そうなので追加する。

```kotlin
/*
 * This file was generated by the Gradle 'init' task.
 *
 * This generated file contains a sample Kotlin library project to get you started.
 * For more details on building Java & JVM projects, please refer to https://docs.gradle.org/8.14.2/userguide/building_java_projects.html in the Gradle documentation.
 */

plugins {
    // Apply the org.jetbrains.kotlin.jvm Plugin to add support for Kotlin.
    alias(libs.plugins.kotlin.jvm)

    // Apply the java-library plugin for API and implementation separation.
    `java-library`

    // Apply the plugin (run-task用に追加)
    id("xyz.jpenilla.run-paper") version "2.3.1"
}

repositories {
    // Use Maven Central for resolving dependencies.
    mavenCentral()

    // Paper用に追加
    maven {
        name = "papermc"
        url = uri("https://repo.papermc.io/repository/maven-public/")
    }
}

dependencies {
    // Use the Kotlin JUnit 5 integration.
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")

    // Use the JUnit 5 integration.
    testImplementation(libs.junit.jupiter.engine)

    testRuntimeOnly("org.junit.platform:junit-platform-launcher")

    // This dependency is exported to consumers, that is to say found on their compile classpath.
    api(libs.commons.math3)

    // This dependency is used internally, and not exposed to consumers on their own compile classpath.
    implementation(libs.guava)

    // Paper用に追加
    compileOnly("io.papermc.paper:paper-api:1.21.8-R0.1-SNAPSHOT")
}

// Apply a specific Java toolchain to ease working on different environments.
java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

tasks.named<Test>("test") {
    // Use JUnit Platform for unit tests.
    useJUnitPlatform()
}


repositories {
}

dependencies {
  compileOnly("io.papermc.paper:paper-api:1.21.8-R0.1-SNAPSHOT")
}

java {
  toolchain.languageVersion.set(JavaLanguageVersion.of(21))
}

// run-task用に追加
// 公式と書き方が異なるので注意
tasks.runServer {
    minecraftVersion("1.21.8") // バージョン指定
}
```

### kotlin でコードを書いてみる

paper のサイトに公開されてるページを kotlin ベースに書き直すとこうなる

```kotlin
package org.adw39.examplePlugin

import net.kyori.adventure.text.Component
import org.bukkit.Bukkit
import org.bukkit.event.Listener
import org.bukkit.event.player.PlayerJoinEvent
import org.bukkit.plugin.java.JavaPlugin

class ExamplePlugin: JavaPlugin(), Listener {
    override fun onEnable() {
        Bukkit.getPluginManager().registerEvents(this, this)
    }

    @EventHandler
    fun onPlayerJoin(event: PlayerJoinEvent) {
        event.player.sendMessage("hello!")
    }

}
```

- 関数の定義は rust などに似てる  
  `fun`を先頭につける。 void 以外の戻り値は、`()`の後ろに`(): <Type>`です。
- `@Override`は`override`を先頭につける  
  しょっちゅう使われるから言語標準で用意されたのかな
- java の`getHogehoge`、`setHogehoge`メソッドは、get/set を省略する  
  いわゆるゲッターセッターってやつです。 kotlin の場合は基本的に省略可能です。

### `plugin.yml`を忘れない

拡張子`yaml`は NG[^参考6]

```yml
name: ExamplePlugin
version: 1.0.0
main: org.adw39.examplePlugin.ExamplePlugin
description: An example plugin
author: nikki9750
website: https://adw39.org
api-version: "1.21.8"
```

### runServer を実行する

サーバーが起動したら成功  
**サーバーの軌道に失敗したら、まず`/lib/run`に生成されるサーバーディレクトリの、`elua.txt`を確認すること**

# 参考

[^参考2]: [kotlin-test](https://kotlinlang.org/api/core/kotlin-test/)
[^参考3]: [Kotest の各種 spec を比べてみた](https://qiita.com/takuya-okamoto/items/97d88f70494f6b695774)
[^参考1]: [Kotlin で始める TDD: JUnit と Kotest によるセットアップ完全ガイド](https://ittrip.xyz/kotlin/kotlin-tdd-setup-guide)
[^参考4]: [英語「unit」の意味・使い方・読み方 | Weblio 英和辞書](https://ejje.weblio.jp/content/unit)
[^参考5]: [Using Tasks](https://docs.gradle.org/current/userguide/tutorial_using_tasks.html)
[^参考6]: [plugin.yml](https://docs.papermc.io/paper/dev/plugin-yml/)
[^参考7]: [FAT JAR を簡単に作る](https://dev.classmethod.jp/articles/make_fat-jar/)
