package com.prezi.prmoji.services.slack

import com.prezi.prmoji.models.Emoji
import com.prezi.prmoji.services.slack.models._
import io.circe.Codec
import io.circe.generic.semiauto.deriveCodec
import io.circe.jawn.decode
import io.circe.syntax.EncoderOps
import zhttp.http._
import zhttp.service.Client
import zhttp.service.Client.ClientParams
import zio.IO

case class AddEmojiPayload(name: Emoji,
                           channel: SlackChannel,
                           timestamp: SlackTimestamp)

object AddEmojiPayload {
  implicit val addEmojiCodec: Codec[AddEmojiPayload] = deriveCodec
}

final case class ProductionSlackClient(httpClient: Client, slackToken: SlackToken) extends Slack {

  override def addEmoji(emoji: Emoji, channel: SlackChannel, timestamp: SlackTimestamp): IO[SlackError, Unit] =
    httpClient.request(
      ClientParams(
        method = Method.POST,
        url = URL.fromString("http://slack.com/api/reactions.add").toOption.get,
        getHeaders = Headers.host("http://slack.com") ++ Headers.contentType(HeaderValues.applicationJson),
        data = HttpData.fromString(AddEmojiPayload(emoji, channel, timestamp).asJson.noSpaces),
      )
    )
      .flatMap(_.getBodyAsString)
      .mapError(SlackError.ClientError)
      .flatMap(
        response =>
          IO.fromEither(decode[SlackResponse](response))
            .mapError(error => SlackError.InvalidResponse(response, error))
      )
      .flatMap {
        case SlackResponse.OK =>
          IO.unit
        case SlackResponse.Error(error) =>
          IO.fail(SlackError.FailedResponse(error))
      }
}
