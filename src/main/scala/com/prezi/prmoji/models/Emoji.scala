package com.prezi.prmoji.models

import com.prezi.prmoji.codecs.{StringValueTypeJsonCodec, ValueType}

final case class Emoji(value: String) extends ValueType

object Emoji extends StringValueTypeJsonCodec[Emoji] {
  val whiteCheckMark: Emoji = Emoji("white_check_mark")
  val speechBalloon: Emoji = Emoji("speech_balloon")
  val noEntry: Emoji = Emoji("no_entry")
  val merged: Emoji = Emoji("merged")
  val wastebasket: Emoji = Emoji("wastebasket")
}
