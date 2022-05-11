package com.prezi.prmoji.persistence.prmessage

import slick.jdbc.H2Profile.api._
import slick.lifted.{Index, ProvenShape, Tag}

import java.sql.Timestamp

object PRMessageTable {
  final class PRMessages(tag: Tag)
      extends Table[PRMessage](_tableTag = tag, _tableName = "pr_messages") {
    override def * : ProvenShape[PRMessage] =
      (
        id,
        insertedAt,
        prUrl,
        messageChannel,
        messageTimestamp
      ) <> (PRMessage.fromTuple, PRMessage.toTuple)

    def idxPrUrl: Index = index("index_pr_url", prUrl)

    def id: Rep[Long] = column[Long]("id", O.PrimaryKey, O.AutoInc)

    def insertedAt: Rep[Timestamp] = column[Timestamp]("inserted_at")

    def prUrl: Rep[String] = column[String]("pr_url")

    def messageChannel: Rep[String] = column[String]("message_channel")

    def messageTimestamp: Rep[String] = column[String]("message_timestamp")
  }

  val table = TableQuery[PRMessages]
}
