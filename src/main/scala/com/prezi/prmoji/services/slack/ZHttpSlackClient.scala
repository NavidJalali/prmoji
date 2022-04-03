package com.prezi.prmoji.services.slack

import com.prezi.prmoji.models.Emoji
import com.prezi.prmoji.services.slack.models._
import zhttp.http._
import zhttp.service.Client.ClientRequest
import zhttp.service.{ChannelFactory, Client, EventLoopGroup}
import zio.IO
import zio.json.{DecoderOps, EncoderOps}

final case class ZHttpSlackClient(slackToken: SlackToken) extends Slack {

  private def makeHeaders() =
    Headers.authorization(slackToken.token) ++
      Headers.host("slack.com") ++
      Headers.contentType(HeaderValues.applicationJson)

  private def post(url: URL, data: String) =
    Client.request(
      ClientRequest(
        url = url,
        method = Method.POST,
        headers = makeHeaders(),
        data = HttpData.fromString(data),
      )
    )
      .flatMap(_.bodyAsString)
      .mapError(SlackError.ClientError)
      .flatMap(
        response =>
          IO.fromEither(response.fromJson[SlackResponse])
            .mapError(error => SlackError.InvalidResponse(response, error))
      )
      .flatMap {
        case SlackResponse.OK =>
          IO.succeed(SlackResponse.OK)
        case SlackResponse.Error(error) =>
          IO.fail(SlackError.FailedResponse(error))
      }
      .provide(ChannelFactory.auto ++ EventLoopGroup.auto())

  override def addEmoji(name: Emoji, channel: SlackChannel, timestamp: SlackTimestamp): IO[SlackError, SlackResponse.OK.type] =
    post(ZHttpSlackClient.reactions, AddEmojiPayload(name, channel, timestamp).toJson)

  override def postMessage(channel: SlackChannel, text: String): IO[SlackError, SlackResponse.OK.type] =
    post(ZHttpSlackClient.postMessage, PostMessagePayload(channel, text).toJson)
}

object ZHttpSlackClient {
  private val reactions: URL = URL.fromString("https://slack.com/api/reactions.add").toOption.get
  private val postMessage: URL = URL.fromString("https://slack.com/api/chat.postMessage").toOption.get
}
