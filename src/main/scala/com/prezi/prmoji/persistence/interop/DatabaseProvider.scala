package com.prezi.prmoji.persistence.interop

import com.typesafe.config.Config
import slick.jdbc.{JdbcBackend, JdbcProfile}
import zio._

trait DatabaseProvider {
  val db: JdbcBackend#Database
  val profile: JdbcProfile
}

object DatabaseProvider {
  val live: ZManaged[Config & JdbcProfile, Throwable, DatabaseProvider] =
    for {
      jdbcProfile <- ZManaged.service[JdbcProfile]
      config <- ZManaged.service[Config]
      dbDef <- ZManaged.acquireReleaseWith(
        ZIO.attempt(jdbcProfile.backend.Database.forConfig("database", config))
      )(db => ZIO.attempt(db.close()).orDie)
    } yield new DatabaseProvider {
      override val db: JdbcBackend#DatabaseDef = dbDef
      override val profile = jdbcProfile
    }
}
