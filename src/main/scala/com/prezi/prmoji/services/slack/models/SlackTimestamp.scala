package com.prezi.prmoji.services.slack.models

case class SlackTimestamp(value: String) extends AnyVal

object SlackTimestamp extends StringValueTypeJsonCodec[SlackTimestamp]