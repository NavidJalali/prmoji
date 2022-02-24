package com.prezi.prmoji.services.slack.models

import zio.json._

final case class PostMessagePayload(channel: SlackChannel, text: String)

object PostMessagePayload {
  implicit val postMessageCodec: JsonCodec[PostMessagePayload] = DeriveJsonCodec.gen[PostMessagePayload]
}
