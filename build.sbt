name := "scala-prmoji"

version := "0.1"

scalaVersion := "2.13.7"

libraryDependencies ++= Seq(
  "dev.zio"                     %% "zio"                          % "1.0.13",
  "dev.zio"                     %% "zio-test"                     % "1.0.13" % Test,
  "dev.zio"                     %% "zio-json"                     % "0.2.0-M3",
  "io.d11"                      %% "zhttp"                        % "1.0.0.0-RC21",
)

testFrameworks += new TestFramework("zio.test.sbt.ZTestFramework")
