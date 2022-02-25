package com.prezi.prmoji.persistence

import com.prezi.prmoji.persistence.interop.DatabaseProvider
import com.prezi.prmoji.persistence.interop.ZIOCompanionSyntax._
import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import zio.{IO, ZIO, ZLayer}

import java.sql.Timestamp
import java.time.Instant

case class SlickPRMessageRepository(db: DatabaseProvider) extends PRMessageRepository {

  import db.profile.api._

  val prMessages = PRMessageTable.table

  def toIO[A](dbio: DBIO[A]): ZIO[Any, Throwable, A] =
    ZIO.fromDBIO(dbio)
      .provide(ZLayer.succeed(db))

  override def getByUrl(prUrl: String): IO[Throwable, Seq[PRMessage]] =
    toIO(prMessages.filter(_.prUrl === prUrl).result)

  override def create(prUrl: String, messageChannel: SlackChannel, messageTimestamp: SlackTimestamp): IO[Throwable, PRMessage] = {
    val prMessageRow = PRMessage(0, Timestamp.from(Instant.now), prUrl, messageChannel, messageTimestamp)
    toIO(prMessages.returning(prMessages.map(_.id)) += prMessageRow).map(id => prMessageRow.copy(id = id))
  }

  override def delete(prUrl: String): IO[Throwable, Int] =
    toIO(prMessages.filter(_.prUrl === prUrl).delete)

  override def deleteBeforeDate(date: Timestamp): IO[Throwable, Int] =
    toIO(prMessages.filter(_.insertedAt < date).delete)

  override def deleteAll(): IO[Throwable, Int] =
    toIO(prMessages.delete)
}
