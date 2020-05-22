#!/bin/bash

psql ${DATABASE_URL} < schema.sql
