package com.prezi.prmoji.services.slack.models

import zio.json.JsonCodec

trait StringValueTypeJsonCodec[A <: { val value: String }] {
  def apply(string: String): A

  lazy implicit val codec: JsonCodec[A] =
    JsonCodec.string.xmap(
      apply,
      _.value
    )
}
