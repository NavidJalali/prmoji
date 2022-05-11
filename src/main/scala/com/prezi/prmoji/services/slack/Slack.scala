package com.prezi.prmoji.services.slack

import com.prezi.prmoji.models.Emoji
import com.prezi.prmoji.services.slack.models.{
  SlackChannel,
  SlackError,
  SlackResponse,
  SlackTimestamp
}
import zio._

trait Slack {
  def addEmoji(
      name: Emoji,
      channel: SlackChannel,
      timestamp: SlackTimestamp
  ): IO[SlackError, SlackResponse.OK.type]

  def postMessage(
      channel: SlackChannel,
      text: String
  ): IO[SlackError, SlackResponse.OK.type]
}

object Slack extends {
  val live = ZLayer.fromFunction(ProductionSlackClient.apply _)

  def addEmoji(
      name: Emoji,
      channel: SlackChannel,
      timestamp: SlackTimestamp
  ): ZIO[Slack, SlackError, SlackResponse.OK.type] =
    ZIO.environmentWithZIO(_.get.addEmoji(name, channel, timestamp))

  def postMessage(
      channel: SlackChannel,
      text: String
  ): ZIO[Slack, SlackError, SlackResponse.OK.type] =
    ZIO.environmentWithZIO(_.get.postMessage(channel, text))

  //  val test = (TestSlackClient.apply _ ).toLayer
}
