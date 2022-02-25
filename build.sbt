name := "scala-prmoji"

version := "0.1"

scalaVersion := "2.13.7"

val versions = new {
  val zioVersion = "2.0.0-RC2"
  val zioJsonVersion = "0.3.0-RC3"
  val zioHttpVersion = "2.0.0-RC4"
  val slickVersion = "3.3.3"
  val mySQLConnectorVersion = "8.0.25"
  val h2Version = "2.1.210"
}

libraryDependencies ++= Seq(
  "dev.zio"                     %% "zio"                          % versions.zioVersion,
  "dev.zio"                     %% "zio-test"                     % versions.zioVersion % Test,
  "dev.zio"                     %% "zio-json"                     % versions.zioJsonVersion,
  "io.d11"                      %% "zhttp"                        % versions.zioHttpVersion,
  "com.typesafe.slick"          %% "slick"                        % versions.slickVersion,
  "com.typesafe.slick"          %% "slick-hikaricp"               % versions.slickVersion,
  "mysql"                       %  "mysql-connector-java"         % versions.mySQLConnectorVersion,
  "com.h2database"              %  "h2"                            % versions.h2Version
)


testFrameworks += new TestFramework("zio.test.sbt.ZTestFramework")
