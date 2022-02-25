package com.prezi.prmoji.persistence.prmessage

import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import zio._

import java.sql.Timestamp

trait PRMessageRepository {
  def getByUrl(prUrl: String): IO[Throwable, Seq[PRMessage]]

  def create(prUrl: String,
             messageChannel: SlackChannel,
             messageTimestamp: SlackTimestamp): IO[Throwable, PRMessage]

  def delete(prUrl: String): IO[Throwable, Int]

  def deleteBeforeDate(date: Timestamp): IO[Throwable, Int]

  def deleteAll(): IO[Throwable, Int]
}

object PRMessageRepository {
  val live = (SlickPRMessageRepository.apply _).toLayer
  val test = (MockPRMessageRepository.apply _).toLayer
}
