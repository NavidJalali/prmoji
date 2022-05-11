package com.prezi.prmoji.services.slack

import com.prezi.prmoji.models.Emoji
import com.prezi.prmoji.services.slack.models._
import zhttp.http._
import zhttp.service.{ChannelFactory, Client, EventLoopGroup}
import zio.{IO, ZIO}
import zio.json.{DecoderOps, EncoderOps}

final case class ProductionSlackClient(slackToken: SlackToken) extends Slack {

  private def makeHeaders() =
    Headers.authorization(slackToken.token) ++
      Headers.host("slack.com") ++
      Headers.contentType(HeaderValues.applicationJson)

  private def post(url: String, data: String) =
    Client
      .request(
        url = url,
        method = Method.POST,
        headers = makeHeaders(),
        content = HttpData.fromString(data)
      )
      .flatMap(_.bodyAsString)
      .mapError(SlackError.ClientError)
      .flatMap(response =>
        ZIO
          .fromEither(response.fromJson[SlackResponse])
          .mapError(error => SlackError.InvalidResponse(response, error))
      )
      .flatMap {
        case SlackResponse.OK =>
          ZIO.succeed(SlackResponse.OK)
        case SlackResponse.Error(error) =>
          ZIO.fail(SlackError.FailedResponse(error))
      }
      .provide(ChannelFactory.auto ++ EventLoopGroup.auto())

  override def addEmoji(
      name: Emoji,
      channel: SlackChannel,
      timestamp: SlackTimestamp
  ): IO[SlackError, SlackResponse.OK.type] =
    post(
      ProductionSlackClient.reactions,
      AddEmojiPayload(name, channel, timestamp).toJson
    )

  override def postMessage(
      channel: SlackChannel,
      text: String
  ): IO[SlackError, SlackResponse.OK.type] =
    post(
      ProductionSlackClient.postMessage,
      PostMessagePayload(channel, text).toJson
    )
}

object ProductionSlackClient {
  private val reactions = "https://slack.com/api/reactions.add"
  private val postMessage = "https://slack.com/api/chat.postMessage"
}
