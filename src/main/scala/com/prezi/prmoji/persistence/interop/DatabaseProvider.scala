package com.prezi.prmoji.persistence.interop

import com.typesafe.config.Config
import slick.jdbc.{JdbcBackend, JdbcProfile}
import zio._

trait DatabaseProvider {
  val db: JdbcBackend#Database
  val profile: JdbcProfile
}

object DatabaseProvider {
  val live: ZLayer[Config & JdbcProfile, Throwable, DatabaseProvider] = {
    ZLayer.fromZIO {
      ZIO.scoped {
        for {
          jdbcProfile <- ZIO.service[JdbcProfile]
          config <- ZIO.service[Config]
          dbDef <- ZIO.acquireRelease(
            ZIO.attempt(jdbcProfile.backend.Database.forConfig("database", config))
          )(db => ZIO.attempt(db.close()).orDie)
        } yield new DatabaseProvider {
          override val db: JdbcBackend#DatabaseDef = dbDef
          override val profile = jdbcProfile
        }
      }
    }
  }
}
