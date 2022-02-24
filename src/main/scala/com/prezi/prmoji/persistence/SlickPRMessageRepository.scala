package com.prezi.prmoji.persistence

import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import slick.interop.zio.DatabaseProvider
import slick.interop.zio.syntax._
import slick.jdbc.{JdbcBackend, JdbcProfile}
import zio.{IO, ZIO, ZLayer}

import java.sql.Timestamp
import java.time.Instant
import scala.util.chaining.scalaUtilChainingOps

case class SlickPRMessageRepository(databaseProvider: DatabaseProvider, jdbcProfile: JdbcProfile) extends PRMessageRepository {

  import jdbcProfile.api._

  val prMessages = PRMessageTable.table

  val dbLayer: ZLayer[Any, Nothing, JdbcBackend#DatabaseDef] = databaseProvider.db.toLayer

  override def getByUrl(prUrl: String): IO[Throwable, Seq[PRMessage]] =
    prMessages
        .filter(_.prUrl === prUrl)
        .result.pipe(ZIO.fromDBIO)
        .provide(dbLayer)

  override def create(prUrl: String, messageChannel: SlackChannel, messageTimestamp: SlackTimestamp): IO[Throwable, PRMessage] = {
    val prMessageRow = PRMessage(0, Timestamp.from(Instant.now()), prUrl, messageChannel, messageTimestamp);

    ZIO.fromDBIO(prMessages.returning(prMessages.map(_.id)) += prMessageRow)
        .provide(dbLayer)
        .map(id => prMessageRow.copy(id = id))
  }

  override def delete(prUrl: String): IO[Throwable, Int] =
    ZIO.fromDBIO(prMessages.filter(_.prUrl === prUrl).delete)
        .provide(dbLayer)

  override def deleteBeforeDate(date: Timestamp): IO[Throwable, Int] = ???

  override def deleteAll(): IO[Throwable, Int] = ZIO.fromDBIO(prMessages.delete)
      .provide(dbLayer)
}
