package com.prezi.prmoji.services.slack

import com.prezi.prmoji.models.Emoji
import com.prezi.prmoji.services.slack.models._
import zhttp.http._
import zhttp.service.Client
import zhttp.service.Client.ClientParams
import zio.IO
import zio.json.{DecoderOps, DeriveJsonCodec, EncoderOps, JsonCodec}

final case class ProductionSlackClient(httpClient: Client, slackToken: SlackToken) extends Slack {

  override def addEmoji(emoji: Emoji, channel: SlackChannel, timestamp: SlackTimestamp): IO[SlackError, Unit] =
    httpClient.request(
      ClientParams(
        method = Method.POST,
        url = URL.fromString("https://slack.com/api/reactions.add").toOption.get,
        getHeaders = Headers.host("slack.com") ++ Headers.contentType(HeaderValues.applicationJson),
        data = HttpData.fromString(AddEmojiPayload(emoji, channel, timestamp).toJson),
      )
    )
      .flatMap(_.getBodyAsString)
      .mapError(SlackError.ClientError)
      .flatMap(
        response =>
          IO.fromEither(response.fromJson[SlackResponse])
            .mapError(error => SlackError.InvalidResponse(response, error))
      )
      .flatMap {
        case SlackResponse.OK =>
          IO.unit
        case SlackResponse.Error(error) =>
          IO.fail(SlackError.FailedResponse(error))
      }
}
