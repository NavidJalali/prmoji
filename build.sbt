name := "scala-prmoji"

version := "0.1"

scalaVersion := "2.13.7"

val zioVersion = "2.0.0-RC1"

val zioJsonVersion = "0.3.0-RC2"

val zioHttpVersion = "2.0.0-RC2"

libraryDependencies ++= Seq(
  "dev.zio"                     %% "zio"                          % zioVersion,
  "dev.zio"                     %% "zio-test"                     % zioVersion % Test,
  "dev.zio"                     %% "zio-json"                     % zioJsonVersion,
  "io.d11"                      %% "zhttp"                        % zioHttpVersion,
  "com.typesafe.slick"          %% "slick"                        % "3.3.3",
  "com.typesafe.slick"          %% "slick-hikaricp"               % "3.3.3",
  "io.scalac"                   %% "zio-slick-interop"            % "0.3.0"
)

libraryDependencies += "mysql" % "mysql-connector-java" % "8.0.23"
libraryDependencies += "com.h2database" % "h2" % "1.4.200"

testFrameworks += new TestFramework("zio.test.sbt.ZTestFramework")
