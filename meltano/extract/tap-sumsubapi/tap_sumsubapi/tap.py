"""SumsubApi tap class."""

from __future__ import annotations

from singer_sdk import Tap
from singer_sdk import typing as th  # JSON schema typing helpers

from tap_sumsubapi.streams import ApplicantStream

STREAM_TYPES = [ApplicantStream]


class TapSumsubApi(Tap):
    """SumsubApi tap class."""

    name = "tap-sumsubapi"

    config_jsonschema = th.PropertiesList(
        th.Property("host", th.StringType, required=True),
        th.Property("port", th.IntegerType, default=5432),
        th.Property("database", th.StringType, required=True),
        th.Property("user", th.StringType, required=True),
        th.Property("password", th.StringType, required=True),
        th.Property("sslmode", th.StringType, default="prefer"),
        th.Property("sumsub_secret_key", th.StringType, required=True),
        th.Property("sumsub_app_token", th.StringType, required=True),
    ).to_dict()

    def discover_streams(self):
        """Return a list of discovered streams."""
        return [stream_class(tap=self) for stream_class in STREAM_TYPES]


if __name__ == "__main__":
    TapSumsubApi.cli()
