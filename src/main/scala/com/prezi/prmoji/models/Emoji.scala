package com.prezi.prmoji.models

import com.prezi.prmoji.codecs.{StringValueTypeJsonCodec, ValueType}

final case class Emoji(value: String) extends ValueType

object Emoji extends StringValueTypeJsonCodec[Emoji] {
  val whiteCheckMark: Emoji = Emoji("white_check_mark")
  val speechBalloon: Emoji = Emoji("speech_balloon")
  val noEntry: Emoji = Emoji("no_entry")
  val merged: Emoji = Emoji("merged")
  val wastebasket: Emoji = Emoji("wastebasket")

  private val defaultGithubMapping: Map[GithubAction, Emoji] = Map(
    GithubAction.Commented -> speechBalloon,
    GithubAction.Approved -> whiteCheckMark,
    GithubAction.ChangesRequested -> noEntry,
    GithubAction.Merged -> merged,
    GithubAction.Closed -> wastebasket
  )

  def fromGithubAction(githubAction: GithubAction,
                       overrides: PartialFunction[GithubAction, Emoji] = PartialFunction.empty): Option[Emoji] =
    overrides.orElse(defaultGithubMapping).lift(githubAction)
}

