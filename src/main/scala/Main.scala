import zio.Console.printLine
import zio.{ExitCode, URIO, ZIOAppDefault}

object Main extends ZIOAppDefault {
  override def run: URIO[zio.ZEnv, ExitCode] =
    printLine("Hello").orDie.exitCode
}
