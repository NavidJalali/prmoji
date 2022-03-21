package com.prezi.prmoji.models

import zio.json.JsonDecoder

sealed trait GithubAction

object GithubAction {
  def apply(tag: String): GithubAction = fromString(tag)

  case object Commented extends GithubAction

  case object Approved extends GithubAction

  case object ChangesRequested extends GithubAction

  case object Merged extends GithubAction

  case object Closed extends GithubAction

  final case class Other(actionTag: String) extends GithubAction

  val fromString: String => GithubAction = (str: String) => str match {
    case "commented" => Commented
    case "approved" => Approved
    case "changes_requested" => ChangesRequested
    case "merged" => Merged
    case "closed" => Closed
    case tag => Other(tag)
  }

  implicit val githubActionDecoder: JsonDecoder[GithubAction] = JsonDecoder.string.map(fromString)
}
