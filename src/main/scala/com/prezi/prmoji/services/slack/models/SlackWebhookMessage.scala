package com.prezi.prmoji.services.slack.models

import zio.json.{DeriveJsonCodec, JsonCodec}

import scala.util.matching.Regex

final case class SlackWebhookMessage(text: String, channel: SlackChannel, timestamp: SlackTimestamp) {
  def getPrUrls: List[String] =
    SlackWebhookMessage.pullRequestRegex.findAllMatchIn(text).toList.map(_.matched)
}

object SlackWebhookMessage {
  val pullRequestRegex: Regex = """(https://github\.com/[\w-_]+/[\w-_]+/pull/\d+)""".r
  implicit val decoder: JsonCodec[SlackWebhookMessage] = DeriveJsonCodec.gen
}
