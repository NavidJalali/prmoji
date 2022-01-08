package com.prezi.prmoji.codecs

import zio.json.JsonCodec

trait ValueType {
  val value: String
}

trait StringValueTypeJsonCodec[A <: ValueType] {
  def apply(string: String): A

  lazy implicit val codec: JsonCodec[A] =
    JsonCodec.string.xmap(
      apply,
      _.value
    )
}
