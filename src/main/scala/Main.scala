import zio.Console.printLine
import zio.ZIOAppDefault

object Main extends ZIOAppDefault {

  import zhttp.http._
  import zhttp.service.Server
  import zio._

  val app = Http.collectZIO[Request] {
    case request @ method -> !! / "testing" =>
      request.bodyAsString
        .tap(resp => printLine((method, resp)))
        .as(Response.text("OK"))
  }

  override def run =
    Server.start(8080, app)
}
