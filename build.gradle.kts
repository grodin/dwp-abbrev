plugins {
  kotlin("multiplatform") version "1.6.10"
  id("org.jetbrains.compose") version "1.1.0"
}

group = "com.omricat.dwp-abbrev"
version = "0.1"

repositories {
  mavenCentral()
  maven("https://maven.pkg.jetbrains.space/public/p/compose/dev")
  google()

}

kotlin {
  js(IR) {
    browser()
    binaries.executable()
  }

  sourceSets {
    val jsMain by getting {
      dependencies {
        implementation(compose.web.core)
        implementation(compose.runtime)
      }
    }
  }
}
