#!/usr/bin/env python3.11

import sqlite3
import sys
from pathlib import Path

import logging

logging.basicConfig(format="%(levelname)s | %(asctime)s | %(filename)s: %(message)s", level=logging.DEBUG)
logging.getLogger('asyncio').setLevel(logging.ERROR)

logger = logging.getLogger(__name__)

if sys.version_info >= (3, 11, 0):
    import tomllib as toml
else:
    import toml

with open('../comet-config.toml') as f:
    config = toml.loads(f.read())

path = Path('sql/')

if not path.exists() or not path.is_dir():
    logging.error("`sql/` directory not found, exiting.")
    sys.exit(1)

schemas = sorted( # sorts it on creation time, not modified
    path.iterdir(),
    key=lambda schema: schema.stat().st_ctime
)

if not schemas:
    logging.error("No schemas found in `sql/`, exiting.")
    sys.exit(1)

success = 0
failure = 0

with sqlite3.connect('../data.db') as conn:
    curr = conn.cursor()

    for schema in schemas:
        logger.info('Running `%s`', schema.name)
        sql = schema.read_text()
        try:
            conn.execute(sql)
            success += 1
        except Exception as err:
            failure += 1
            logging.error("Could not execute `%s` due to '%s'", schema.name, err) # don't wanna stop the migration run if the change is already applied.

    conn.commit()
    curr.close()

logging.info('Done! Successfully ran `%s` migrations with `%s` failures.', success, failure)
