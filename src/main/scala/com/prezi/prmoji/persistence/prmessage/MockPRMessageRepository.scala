package com.prezi.prmoji.persistence.prmessage

import com.prezi.prmoji.persistence.prmessage.PRMessageRepository.Error._
import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import zio.IO

import java.sql.Timestamp
import java.time.Instant
import java.util.concurrent.ConcurrentHashMap
import scala.jdk.CollectionConverters._
import scala.math.Ordered.orderingToOrdered

final case class MockPRMessageRepository() extends PRMessageRepository {

  val data = new ConcurrentHashMap[Long, PRMessage]()

  override def getByUrl(prUrl: String): IO[ReadError, Seq[PRMessage]] =
    IO.attempt {
      data
        .asScala
        .collect { case (_, value) if value.prUrl == prUrl => value }
        .toList
    }
      .mapError(ReadError)

  override def create(prUrl: String, messageChannel: SlackChannel, messageTimestamp: SlackTimestamp): IO[WriteError, PRMessage] = {
    IO.attempt {
      val id = scala.util.Random.nextLong()
      val now = Timestamp.from(Instant.now())
      data.put(id, PRMessage(
        id = id, insertedAt = now, prUrl = prUrl, messageChannel = messageChannel, messageTimestamp = messageTimestamp
      ))
    }
  }
    .mapError(WriteError)

  override def createAll(prs: List[(String, SlackChannel, SlackTimestamp)]): IO[WriteError, Unit] =
    IO.foreach(prs) {
      (create _).tupled(_)
    }.unit

  private def deleteIf(predicate: PRMessage => Boolean): IO[DeleteError, Int] =
    IO.attempt {
      val keys = data.asScala.collect { case (key, value) if predicate(value) => key }.toList
      keys.foreach(data.remove)
      keys.length
    }.mapError(DeleteError)

  override def delete(prUrl: String): IO[DeleteError, Int] = deleteIf(_.prUrl == prUrl)

  override def deleteBeforeDate(date: Timestamp): IO[DeleteError, Int] = deleteIf(_.insertedAt < date)

  override def deleteAll(): IO[DeleteError, Int] = deleteIf(_ => true)
}
