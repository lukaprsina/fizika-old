#!/bin/bash

rm ./prisma/db.sqlite && npx prisma db push && npx prisma db seed