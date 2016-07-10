\set ECHO none
BEGIN;
\i sql/natural_sort.sql
\set ECHO all

-- You should write your tests

SELECT natural_compare('test10', 'test1'), natural_compare('test1', 'test10'), natural_compare('test1', 'test1');

ROLLBACK;
