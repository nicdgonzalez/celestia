repositories {
  maven {
    name = "papermc"
    url = uri("https://repo.papermc.io/repository/maven-public/")
  }
}

dependencies {
  compileOnly("io.papermc.paper:paper-api:1.21.10-R0.1-SNAPSHOT")
}

java {
  toolchain.languageVersion.set(JavaLanguageVersion.of(21))
}

tasks.jar {
  manifest {
    attributes["paperweight-mappings-namespace"] = "mojang"
  }
}

plugins {
  java
  // Test plugins in a temporary Minecraft server.
  id("xyz.jpenilla.run-paper") version "3.0.2"
}

tasks {
  runServer {
    // Should match the version of Paper being used in `dependencies`.
    minecraftVersion("1.21.10")
  }
}
