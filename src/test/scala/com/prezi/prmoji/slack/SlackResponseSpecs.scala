package com.prezi.prmoji.slack

import com.prezi.prmoji.services.slack.models.SlackResponse
import com.prezi.prmoji.services.slack.models.SlackResponse.{Error, OK}
import zio.json.DecoderOps
import zio.test.Assertion._
import zio.test.{DefaultRunnableSpec, TestEnvironment, ZSpec, assert}

object SlackResponseSpecs extends DefaultRunnableSpec {
  override def spec: ZSpec[TestEnvironment, Any] =
    suite("Slack Response Specs")(
      test("Can parse successful slack response") {
        val decoded = """{ "ok": true }""".fromJson[SlackResponse]
        assert(decoded)(isRight(equalTo(OK)))
      },

      test("Can parse error slack response") {
        val decoded = """{ "ok": false, "error": "such error" }""".fromJson[SlackResponse]
        assert(decoded)(isRight(equalTo(Error("such error"))))
      },

      test("Fails if ok is false but no error is present") {
        val decoded = """{ "ok": false }""".fromJson[SlackResponse]
        assert(decoded)(isLeft(anything))
      }
    )
}
