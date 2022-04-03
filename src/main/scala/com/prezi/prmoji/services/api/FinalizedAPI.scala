package com.prezi.prmoji.services.api

import com.prezi.prmoji.models.GithubEvent
import zio.IO

final case class ProductionAPI() extends API {
  override def healthcheck(): IO[Nothing, String] =
    IO.succeed("OK")

  override def githubEvent(event: GithubEvent): IO[API.Error, String] =


  override def slackEvent: IO[API.Error, String] = ???

  override def cleanupRequest: IO[API.Error, String] = ???
}
