package com.prezi.prmoji.slack

import com.prezi.prmoji.services.slack.models.SlackResponse
import com.prezi.prmoji.services.slack.models.SlackResponse.{Error, OK}
import io.circe.DecodingFailure
import io.circe.jawn.decode
import zio.test.Assertion._
import zio.test.{DefaultRunnableSpec, ZSpec, assert}

object SlackResponseSpecs extends DefaultRunnableSpec {
  override def spec: ZSpec[_root_.zio.test.environment.TestEnvironment, Any] =
    suite("Slack Response Specs")(
      test("Can parse successful slack response") {
        val decoded = decode[SlackResponse]("""{ "ok": true }""")
        assert(decoded)(isRight(equalTo(OK)))
      },

      test("Can parse error slack response") {
        val decoded = decode[SlackResponse]("""{ "ok": false, "error": "such error" }""")
        assert(decoded)(isRight(equalTo(Error("such error"))))
      },

      test("Fails if ok is false but no error is present") {
        val decoded = decode[SlackResponse]("""{ "ok": false }""")
        assert(decoded)(isLeft(isSubtype[DecodingFailure](anything)))
      }
    )
}
