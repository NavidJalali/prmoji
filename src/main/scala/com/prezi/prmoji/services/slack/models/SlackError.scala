package com.prezi.prmoji.services.slack.models

sealed trait SlackError

object SlackError {
  final case class ClientError(cause: Throwable) extends SlackError
}

