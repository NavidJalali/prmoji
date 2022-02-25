package com.prezi.prmoji.services.api

import zio.IO

trait API[+E] {
  def healthcheck(): IO[E, String]
  def githubEvent(): IO[E, String]
  def slackEvent: IO[E, String]
  def cleanupRequest: IO[E, String]
}
