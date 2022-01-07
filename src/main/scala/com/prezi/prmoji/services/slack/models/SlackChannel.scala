package com.prezi.prmoji.services.slack.models

case class SlackChannel(value: String) extends AnyVal

object SlackChannel extends StringValueTypeJsonCodec[SlackChannel]