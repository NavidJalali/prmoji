package com.prezi.prmoji.persistence.prmessage

import com.prezi.prmoji.persistence.prmessage.PRMessageRepository.Error._
import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}
import zio.{Chunk, IO}
import zio.stm.{TMap, ZSTM}

import java.sql.Timestamp
import java.time.Instant
import scala.util.Random

final case class MockPRMessageRepository(
    underlying: TMap[String, Seq[PRMessage]]
) extends PRMessageRepository {

  override def getByUrl(prUrl: String): IO[ReadError, Seq[PRMessage]] =
    underlying.get(prUrl).map(_.getOrElse(Seq.empty)).commit

  override def create(
      prUrl: String,
      messageChannel: SlackChannel,
      messageTimestamp: SlackTimestamp
  ): IO[WriteError, PRMessage] =
    (for {
      id <- ZSTM.succeed(Random.nextLong)
      insertedAt <- ZSTM.succeed(Timestamp.from(Instant.now()))
      inserted = PRMessage(
        id,
        insertedAt,
        prUrl,
        messageChannel,
        messageTimestamp
      )
      _ <- underlying.updateWith(prUrl) { maybeCurrent =>
        Some(
          maybeCurrent.getOrElse(Seq.empty) :+ inserted
        )
      }
    } yield inserted).commit

  override def delete(prUrl: String): IO[DeleteError, Int] =
    (for {
      count <- underlying.get(prUrl).map(_.fold(0)(_.length))
      _ <- underlying.delete(prUrl)
    } yield count).commit

  override def deleteBeforeDate(date: Timestamp): IO[DeleteError, Int] =
    (for {
      toDelete <- underlying
        .findAll { case id -> messages =>
          Chunk.fromIterable(messages.zipWithIndex.collect {
            case pr -> index if pr.insertedAt.before(date) =>
              id -> index
          })
        }
        .map {
          _.flatten.groupBy(_._1).map { case url -> chunk =>
            url -> chunk.map(_._2).toSet
          }
        }
        .map(Chunk.fromIterable)

      _ <- ZSTM.foreach(toDelete) { case id -> indexes =>
        underlying.updateWith(id) {
          _.map {
            _.zipWithIndex
              .filterNot { case _ -> index =>
                indexes.contains(index)
              }
              .map(_._1)
          }.flatMap(prs => Option.when(prs.nonEmpty)(prs))
        }
      }
    } yield toDelete.foldLeft(0) { case (acc, _ -> chunk) =>
      acc + chunk.size
    }).commit

  override def deleteAll(): IO[DeleteError, Int] =
    (for {
      keys <- underlying.keys
      count = keys.length
      _ <- underlying.deleteAll(keys)
    } yield count).commit

  override def createAll(
      prs: List[(String, SlackChannel, SlackTimestamp)]
  ): IO[WriteError, Unit] = {
    val now = Timestamp.from(Instant.now)
    val toInsert = prs.groupBy(_._1).toList.map { case (url, prs) =>
      url -> prs.map { case (_, channel, timestamp) =>
        PRMessage(
          Random.nextLong,
          now,
          url,
          channel,
          timestamp
        )
      }
    }
    ZSTM
      .foreach(toInsert) { case (url, prs) =>
        underlying.updateWith(url) {
          _.map(prs ++ _).map(_.distinct)
        }
      }
      .unit
      .commit
  }
}
