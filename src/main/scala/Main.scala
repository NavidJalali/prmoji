import com.prezi.prmoji.services.slack.Slack
import zio.ZIO

object Main {
  def main(args: Array[String]) = {
    val effect = for {
      _ <- Slack(i => i.addEmoji(???, ???, ???))
    } yield ()

    zio.Runtime.global.unsafeRun()

    //    for {
    //      slackClient <- ZIO.service[Slack]
    //      _ <- slackClient.addEmoji(???, )
    //    } yield slackClient
  }
}