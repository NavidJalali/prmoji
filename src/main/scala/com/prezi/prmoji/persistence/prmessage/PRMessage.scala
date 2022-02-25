package com.prezi.prmoji.persistence.prmessage

import com.prezi.prmoji.services.slack.models.{SlackChannel, SlackTimestamp}

import java.sql.Timestamp

case class PRMessage(id: Long,
                     insertedAt: Timestamp,
                     prUrl: String,
                     messageChannel: SlackChannel,
                     messageTimestamp: SlackTimestamp)

object PRMessage {
  type PRMessageTuple = (Long, Timestamp, String, String, String)

  def fromTuple(tuple: PRMessageTuple): PRMessage =
    tuple match {
      case (id, insertedAt, prUrl, channel, ts) =>
        PRMessage(id, insertedAt, prUrl, SlackChannel(channel), SlackTimestamp(ts))
    }

  def toTuple(prMessage: PRMessage): Option[PRMessageTuple] =
    prMessage match {
      case PRMessage(id, insertedAt, prUrl, messageChannel, messageTimestamp) =>
        Some(id, insertedAt, prUrl, messageChannel.value, messageTimestamp.value)
    }
}
