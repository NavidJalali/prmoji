package com.prezi.prmoji.services.slack.models

import com.prezi.prmoji.codecs.{StringValueTypeJsonCodec, ValueType}

final case class SlackChannel(value: String) extends ValueType

object SlackChannel extends StringValueTypeJsonCodec[SlackChannel]
