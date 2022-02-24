package com.prezi.prmoji.persistence

import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import slick.lifted.{Index, ProvenShape, Tag}
import zio.IO
import slick.jdbc.H2Profile.api._

import java.sql.Timestamp


case class PRMessage(id: Long,
                     insertedAt: Timestamp,
                     prUrl: String,
                     messageChannel: SlackChannel,
                     messageTimestamp: SlackTimestamp)

object PRMessage {
  type PRMessageTuple = (Long, Timestamp, String, String, String)

  def fromTuple(tuple: PRMessageTuple): PRMessage = ???

  def toTuple(prMessage: PRMessageTuple): Option[PRMessageTuple] = ???
}

trait PRMessageRepository {
  def getByUrl(prUrl: String): IO[Throwable, Seq[PRMessage]]

  def create(prUrl: String,
             messageChannel: SlackChannel,
             messageTimestamp: SlackTimestamp): IO[Throwable, PRMessage]

  def delete(prUrl: String): IO[Throwable, Int]

  def deleteBeforeDate(date: Timestamp): IO[Throwable, Int]

  def deleteAll(): IO[Throwable, Int]
}


object PRMessageTable {
  final class PRMessages(tag: Tag) extends Table[PRMessage](_tableTag = tag, _tableName = "pr_messages") {
    override def * : ProvenShape[PRMessage] =
      (id, insertedAt, prUrl, messageChannel, messageTimestamp) <> (PRMessage.fromTuple, PRMessage.toTuple)

    def idxPrUrl: Index = index("index_pr_url", prUrl)

    def id: Rep[Long] = column[Long]("id", O.PrimaryKey, O.AutoInc)

    def insertedAt: Rep[Timestamp] = column[Timestamp]("inserted_at")

    def prUrl: Rep[String] = column[String]("pr_url")

    def messageChannel: Rep[String] = column[String]("message_channel")

    def messageTimestamp: Rep[String] = column[String]("message_timestamp")
  }

  val table = TableQuery[PRMessages]
}
