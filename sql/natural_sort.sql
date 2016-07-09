/*
 * Author: stereobooster <stereobooster@gmail.com>
 * Created at: 2016-07-09 21:43:30 +0300
 *
 */

--
-- This is a example code genereted automaticaly
-- by pgxn-utils.

-- This is how you define a C function in PostgreSQL.
CREATE OR REPLACE FUNCTION natural_sort(text)
RETURNS text
AS 'natural_sort'
LANGUAGE C IMMUTABLE STRICT;

-- See more: http://www.postgresql.org/docs/current/static/xfunc-c.html
