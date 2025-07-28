# 基礎編 part2

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

# 参考

[^参考2]: [kotlin-test](https://kotlinlang.org/api/core/kotlin-test/)
[^参考3]: [Kotest の各種 spec を比べてみた](https://qiita.com/takuya-okamoto/items/97d88f70494f6b695774)
[^参考1]: [Kotlin で始める TDD: JUnit と Kotest によるセットアップ完全ガイド](https://ittrip.xyz/kotlin/kotlin-tdd-setup-guide)
[^参考4]: [英語「unit」の意味・使い方・読み方 | Weblio 英和辞書](https://ejje.weblio.jp/content/unit)
