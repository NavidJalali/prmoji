package com.prezi.prmoji.persistence.interop

import slick.dbio.DBIO
import zio.ZIO

import scala.concurrent.ExecutionContext

object ZIOCompanionSyntax {

  private def getDb = ZIO.service[DatabaseProvider].map(_.db)

  implicit class ZIOCompanionOps(private val companion: ZIO.type) extends AnyVal {

    def fromDBIO[A](f: ExecutionContext => DBIO[A]): ZIO[DatabaseProvider, Throwable, A] =
      for {
        db <- getDb
        a <- ZIO.fromFuture(ec => db.run(f(ec)))
      } yield a

    def fromDBIO[A](dbio: => DBIO[A]): ZIO[DatabaseProvider, Throwable, A] =
      for {
        db <- getDb
        a <- ZIO.fromFuture(_ => db.run(dbio))
      } yield a
  }
}
