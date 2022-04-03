import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp, SlackWebhookMessage}
import zio.Console.printLine
import zio.ZIOAppDefault

object Main extends ZIOAppDefault {

  import zhttp.http._
  import zhttp.service.Server
  import zio._

  val app = Http.collectZIO[Request] {
    case request@method -> !! / "testing" =>
      request.bodyAsString
        .tap(resp => printLine((method, resp)))
        .as(Response.text("OK"))
  }

  val server = Server.start(8080, app)

  override def run: URIO[zio.ZEnv, ExitCode] =
    ZIO.attempt {
      SlackWebhookMessage("", text, SlackChannel(""), SlackTimestamp("")).getPrUrls
    }
      .tap(Console.printLine(_))
      .catchAllCause(Console.printLine(_))
      .exitCode
}
