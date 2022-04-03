package com.prezi.prmoji.models

import zio.json.{JsonCodec}

final case class GithubEvent(action: GithubAction, pullRequest: PullRequest)

object GithubEvent {
  implicit val codec: JsonCodec[GithubEvent] = DeriveJsonCodec.gen
}
