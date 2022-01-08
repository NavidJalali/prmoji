package com.prezi.prmoji.services.slack.models

import com.prezi.prmoji.codecs.{StringValueTypeJsonCodec, ValueType}

final case class SlackTimestamp(value: String) extends ValueType

object SlackTimestamp extends StringValueTypeJsonCodec[SlackTimestamp]