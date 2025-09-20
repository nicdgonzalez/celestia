repositories {
  maven {
    name = "papermc"
    url = uri("https://repo.papermc.io/repository/maven-public/")
  }
}

dependencies {
  compileOnly("io.papermc.paper:paper-api:1.21.8-R0.1-SNAPSHOT")
  implementation("com.fasterxml.jackson.core:jackson-databind:2.19.0")
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
  id("xyz.jpenilla.run-paper") version "2.3.1"
}

tasks {
  runServer {
    // Configure the Minecraft version for our task.
    // This is the only required configuration besides applying the plugin.
    // Your plugin's jar (or shadowJar if present) will be used automatically.
    minecraftVersion("1.21.8")
  }
}
