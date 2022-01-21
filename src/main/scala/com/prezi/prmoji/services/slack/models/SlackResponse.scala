package com.prezi.prmoji.services.slack.models

import io.circe.{Codec, HCursor, Json}

sealed trait SlackResponse {
  val ok: Boolean
}

object SlackResponse {
  case object OK extends SlackResponse {
    override val ok: Boolean = true
  }

  case class Error(error: String) extends SlackResponse {
    override val ok: Boolean = false
  }

  implicit val codec: Codec[SlackResponse] =
    Codec.from(
      (c: HCursor) => for {
        ok <- c.downField("ok").as[Boolean]
        result <- if (ok) Right(OK) else c.downField("error").as[String].map(Error)
      } yield result,
      {
        case OK => Json.fromFields(
          List("ok" -> Json.True)
        )
        case Error(error) => Json.fromFields(List(
          "ok" -> Json.False,
          "error" -> Json.fromString(error)
        ))
      }
    )
}
