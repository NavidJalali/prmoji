package com.prezi.prmoji.persistence.prmessage

import com.prezi.prmoji.persistence.prmessage.PRMessageRepository.Error._
import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import zio._

import java.sql.Timestamp

trait PRMessageRepository {
  def getByUrl(prUrl: String): IO[ReadError, Seq[PRMessage]]

  def create(prUrl: String,
             messageChannel: SlackChannel,
             messageTimestamp: SlackTimestamp): IO[WriteError, PRMessage]

  def createAll(prs: List[(String, SlackChannel, SlackTimestamp)]): IO[WriteError, Unit]

  def delete(prUrl: String): IO[DeleteError, Int]

  def deleteBeforeDate(date: Timestamp): IO[DeleteError, Int]

  def deleteAll(): IO[DeleteError, Int]
}

object PRMessageRepository {
  val live = (SlickPRMessageRepository.apply _).toLayer
  val test = (MockPRMessageRepository.apply _).toLayer

  sealed trait Error {
    val cause: Throwable
  }

  object Error {
    final case class ReadError(cause: Throwable) extends Error

    final case class WriteError(cause: Throwable) extends Error

    final case class DeleteError(cause: Throwable) extends Error
  }
}
