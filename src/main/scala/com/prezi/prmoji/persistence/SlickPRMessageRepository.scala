package com.prezi.prmoji.persistence

import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import slick.interop.zio.DatabaseProvider
import slick.interop.zio.syntax._
import slick.jdbc.{JdbcBackend, JdbcProfile}
import zio.{Function2ToLayerOps, IO, ZIO, ZLayer}

import java.sql.Timestamp
import java.time.Instant

case class SlickPRMessageRepository(databaseProvider: DatabaseProvider, jdbcProfile: JdbcProfile) extends PRMessageRepository {

  import jdbcProfile.api._

  val prMessages = PRMessageTable.table

  val dbLayer: ZLayer[Any, Nothing, JdbcBackend#DatabaseDef] = databaseProvider.db.toLayer

  def toIO[A](dbio: DBIO[A]): ZIO[Any, Throwable, A] = ZIO.fromDBIO(dbio).provide(dbLayer)

  override def getByUrl(prUrl: String): IO[Throwable, Seq[PRMessage]] =
    toIO(prMessages.filter(_.prUrl === prUrl).result)

  override def create(prUrl: String, messageChannel: SlackChannel, messageTimestamp: SlackTimestamp): IO[Throwable, PRMessage] = {
    val prMessageRow = PRMessage(0, Timestamp.from(Instant.now()), prUrl, messageChannel, messageTimestamp)
    toIO(prMessages.returning(prMessages.map(_.id)) += prMessageRow).map(id => prMessageRow.copy(id = id))
  }

  override def delete(prUrl: String): IO[Throwable, Int] =
    toIO(prMessages.filter(_.prUrl === prUrl).delete)

  override def deleteBeforeDate(date: Timestamp): IO[Throwable, Int] =
    toIO(prMessages.filter(_.insertedAt < date).delete)

  override def deleteAll(): IO[Throwable, Int] =
    toIO(prMessages.delete)
}
