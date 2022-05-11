package com.prezi.prmoji.persistence.prmessage

import com.prezi.prmoji.persistence.interop.DatabaseProvider
import com.prezi.prmoji.persistence.interop.ZIOCompanionSyntax._
import com.prezi.prmoji.persistence.prmessage.PRMessageRepository.Error._
import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import zio.{IO, ULayer, ZIO, ZLayer}

import java.sql.Timestamp
import java.time.Instant

case class SlickPRMessageRepository(db: DatabaseProvider)
    extends PRMessageRepository {

  import db.profile.api._

  val prMessages = PRMessageTable.table

  val env: ULayer[DatabaseProvider] = ZLayer.succeed(db)

  def toIO[A](dbio: DBIO[A]): ZIO[Any, Throwable, A] =
    ZIO.fromDBIO(dbio).provide(env)

  override def getByUrl(prUrl: String): IO[ReadError, Seq[PRMessage]] =
    toIO(prMessages.filter(_.prUrl === prUrl).result)
      .mapError(ReadError)

  override def create(
      prUrl: String,
      messageChannel: SlackChannel,
      messageTimestamp: SlackTimestamp
  ): IO[WriteError, PRMessage] = {
    val prMessageRow = PRMessage(
      0,
      Timestamp.from(Instant.now),
      prUrl,
      messageChannel,
      messageTimestamp
    )
    toIO(prMessages.returning(prMessages.map(_.id)) += prMessageRow)
      .mapBoth(WriteError, id => prMessageRow.copy(id = id))
  }

  override def delete(prUrl: String): IO[DeleteError, Int] =
    toIO(prMessages.filter(_.prUrl === prUrl).delete)
      .mapError(DeleteError)

  override def deleteBeforeDate(date: Timestamp): IO[DeleteError, Int] =
    toIO(prMessages.filter(_.insertedAt < date).delete)
      .mapError(DeleteError)

  override def deleteAll(): IO[DeleteError, Int] =
    toIO(prMessages.delete)
      .mapError(DeleteError)

  override def createAll(
      prs: List[(String, SlackChannel, SlackTimestamp)]
  ): IO[WriteError, Unit] = {
    val now = Timestamp.from(Instant.now)
    val rows = prs.map { case (url, channel, timestamp) =>
      PRMessage(0, now, url, channel, timestamp)
    }
    toIO(prMessages ++= rows).unit
      .mapError(WriteError)
  }
}
