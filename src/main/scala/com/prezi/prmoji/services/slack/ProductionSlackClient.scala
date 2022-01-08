package com.prezi.prmoji.services.slack

import com.prezi.prmoji.models.Emoji
import com.prezi.prmoji.services.slack.models.SlackError.ClientError
import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackError, SlackTimestamp, SlackToken}
import zhttp.http.{HeaderValues, Headers, HttpData, Method, URL}
import zhttp.service.Client
import zhttp.service.Client.ClientParams
import zio.IO
import zio.json.{DeriveJsonCodec, EncoderOps, JsonCodec}

case class AddEmojiPayload(name: Emoji,
                           channel: SlackChannel,
                           timestamp: SlackTimestamp)
object AddEmojiPayload {
  implicit val addEmojiCodec: JsonCodec[AddEmojiPayload] = DeriveJsonCodec.gen
}

final case class ProductionSlackClient(httpClient: Client, slackToken: SlackToken) extends Slack {

  override def addEmoji(emoji: Emoji, channel: SlackChannel, timestamp: SlackTimestamp): IO[SlackError, Unit] = {

    httpClient.request(
      ClientParams(
        method = Method.POST,
        url = URL.fromString("http://slack.com/api/reactions.add").toOption.get,
        getHeaders = Headers.host("http://slack.com") ++ Headers.contentType(HeaderValues.applicationJson),
        data = HttpData.fromString(AddEmojiPayload(emoji, channel, timestamp).toJson),
      )
    ).mapError(ClientError)

    ???
  }
}
