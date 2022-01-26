package com.prezi.prmoji.services.slack.models

import com.prezi.prmoji.models.Emoji
import zio.json._

final case class AddEmojiPayload(name: Emoji, channel: SlackChannel, timestamp: SlackTimestamp)

object AddEmojiPayload {
  implicit val addEmojiCodec: JsonCodec[AddEmojiPayload] = DeriveJsonCodec.gen[AddEmojiPayload]
}
