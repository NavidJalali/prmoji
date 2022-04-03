package com.prezi.prmoji.models

import zio.json.ast.{Json, JsonCursor}
import zio.json.JsonDecoder

final case class User(login: String, avatarUrl: String)

object User {
  implicit val decoder: JsonDecoder[User] = Json.decoder.mapOrFail {
    json =>
      for {
        login <- json.get(JsonCursor.field("login")).flatMap(_.as[String])
        avatarUrl <- json.get(JsonCursor.field("avatar_url")).flatMap(_.as[String])
      } yield User(login, avatarUrl)
  }
}
