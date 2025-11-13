plugins {
    kotlin("jvm") version "1.9.0"
    application
    `maven-publish`
}



group = "org.unicode.icu4x"
version = "2.0-SNAPSHOT"

repositories {
    mavenCentral()
}

// declare a "configuration" named "someConfiguration"
val someConfiguration by configurations.creating

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0")
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.9.2")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
    testImplementation("io.mockk:mockk:1.13.10")
    testImplementation("net.jqwik:jqwik-kotlin:1.9.3")
}
publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "dev.diplomattest"
            artifactId = "icu4x"
            version = "2.0-SNAPSHOT"

            from(components["java"])
        }
    }
}







tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(17)
}

application {
}
