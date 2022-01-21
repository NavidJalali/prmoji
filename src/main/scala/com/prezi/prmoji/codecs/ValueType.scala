package com.prezi.prmoji.codecs

import io.circe.{Codec, Decoder, Encoder}

trait ValueType {
  val value: String
}

trait StringValueTypeJsonCodec[A <: ValueType] {
  def apply(string: String): A

  lazy implicit val codec: Codec[A] =
    Codec.from(
      Decoder.decodeString.map(apply),
      Encoder.encodeString.contramap(_.value)
    )
}
