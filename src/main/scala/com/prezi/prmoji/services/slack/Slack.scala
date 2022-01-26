package com.prezi.prmoji.services.slack

import com.prezi.prmoji.models.Emoji
import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackError, SlackTimestamp}
import zio.{Accessible, Function2ToLayerOps, IO}

trait Slack {
  def addEmoji(name: Emoji, channel: SlackChannel, timestamp: SlackTimestamp): IO[SlackError, Unit]
}

object Slack extends Accessible[Slack] {
  val live = (ProductionSlackClient.apply _).toLayer

  //  val test = (TestSlackClient.apply _ ).toLayer
}