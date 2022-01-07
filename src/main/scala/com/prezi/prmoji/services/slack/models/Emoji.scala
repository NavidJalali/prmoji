package com.prezi.prmoji.services.slack.models

case class Emoji(value: String) extends AnyVal

object Emoji extends StringValueTypeJsonCodec[Emoji] {
  val greenTickmark: Emoji = Emoji(":green_tickmark:")
}


