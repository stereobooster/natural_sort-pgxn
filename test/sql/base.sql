\set ECHO 0
BEGIN;
\i sql/natural_sort.sql
\set ECHO all

-- You should write your tests

SELECT natural_sort('test');

ROLLBACK;
