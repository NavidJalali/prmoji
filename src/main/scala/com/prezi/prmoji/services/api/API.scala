package com.prezi.prmoji.services.api

import zio.IO
import API.Error
import com.prezi.prmoji.models.GithubEvent

trait API {
  def healthcheck(): IO[Nothing, String]
  def githubEvent(event: GithubEvent): IO[Error, String]
  def slackEvent: IO[Error, String]
  def cleanupRequest: IO[Error, String]
}

object API {
  case class Error(statusCode: Int, message: String)
}
