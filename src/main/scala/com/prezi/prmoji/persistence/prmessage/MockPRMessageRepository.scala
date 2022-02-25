package com.prezi.prmoji.persistence.prmessage

import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import zio.IO

import java.sql.Timestamp
import java.time.Instant
import java.util.concurrent.ConcurrentHashMap
import scala.jdk.CollectionConverters._
import scala.math.Ordered.orderingToOrdered

final class MockPRMessageRepository extends PRMessageRepository {

  val data = new ConcurrentHashMap[Long, PRMessage]()

  override def getByUrl(prUrl: String): IO[Throwable, Seq[PRMessage]] =
    IO.attempt {
      data
        .asScala
        .collect { case (_, value) if value.prUrl == prUrl => value }
        .toList
    }

  override def create(prUrl: String, messageChannel: SlackChannel, messageTimestamp: SlackTimestamp): IO[Throwable, PRMessage] =
    IO.attempt {
      val id = scala.util.Random.nextLong()
      val now = Timestamp.from(Instant.now())
      data.put(id, PRMessage(
        id = id, insertedAt = now, prUrl = prUrl, messageChannel = messageChannel, messageTimestamp = messageTimestamp
      ))
    }

  private def deleteIf(predicate: PRMessage => Boolean): IO[Throwable, Int] =
    IO.attempt {
      val keys = data.asScala.collect { case (key, value) if predicate(value) => key }.toList
      keys.foreach(data.remove)
      keys.length
    }

  override def delete(prUrl: String): IO[Throwable, Int] = deleteIf(_.prUrl == prUrl)

  override def deleteBeforeDate(date: Timestamp): IO[Throwable, Int] = deleteIf(_.insertedAt < date)

  override def deleteAll(): IO[Throwable, Int] = deleteIf(_ => true)
}

object MockPRMessageRepository {
  def apply(): MockPRMessageRepository = new MockPRMessageRepository
}