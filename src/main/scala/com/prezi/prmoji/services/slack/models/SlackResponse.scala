package com.prezi.prmoji.services.slack.models

import zio.json.ast.{Json, JsonCursor}
import zio.json.JsonCodec

sealed trait SlackResponse extends Product with Serializable { self =>
  val ok: Boolean
  def fold[A, B](ifOK: => A)(ifError: String => A): A = self match {
    case SlackResponse.OK => ifOK
    case SlackResponse.Error(error) => ifError(error)
  }
}

object SlackResponse {
  case object OK extends SlackResponse {
    override val ok: Boolean = true
  }

  case class Error(error: String) extends SlackResponse {
    override val ok: Boolean = false
  }

  implicit val codec: JsonCodec[SlackResponse] = JsonCodec(
    Json.encoder.contramap[SlackResponse]({
      case OK => Json.Obj("ok" -> Json.Bool(true))
      case Error(error) => Json.Obj("ok" -> Json.Bool(false), "error" -> Json.Str(error))
    }),
    Json.decoder.mapOrFail(json => for {
      ok <- json.get(JsonCursor.field("ok")).flatMap(_.as[Boolean])
      result <-
        if (ok) Right(OK)
        else json.get(JsonCursor.field("error")).flatMap(_.as[String]).map(Error)
    } yield result)
  )
}
