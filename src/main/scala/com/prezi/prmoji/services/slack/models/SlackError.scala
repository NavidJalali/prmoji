package com.prezi.prmoji.services.slack.models

sealed trait SlackError

object SlackError {
  final case class ClientError(cause: Throwable) extends SlackError
  final case class InvalidResponse(response: String, error: io.circe.Error) extends SlackError
  final case class FailedResponse(error: String) extends SlackError
}
