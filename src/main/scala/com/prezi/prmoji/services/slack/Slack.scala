package com.prezi.prmoji.services.slack

import com.prezi.prmoji.services.slack.models.{Emoji, SlackChannel, SlackError, SlackTimestamp}
import zio.IO
import zio._

trait Slack {
  def addEmoji(name: Emoji, channel: SlackChannel, timestamp: SlackTimestamp): IO[SlackError, Unit]
}

object Slack extends Accessible[Slack] {
  val live = (ProductionSlackClient.apply _).toLayer

//  val test = (TestSlackClient.apply _ ).toLayer
}